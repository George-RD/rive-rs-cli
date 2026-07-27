use crate::render::RenderError;
use serde_json::{Value, json};
use std::net::TcpStream;
use std::{
    env, fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tungstenite::{Message, WebSocket, connect};

pub struct Chrome {
    child: Child,
    socket: WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>,
    pub session: String,
    profile: PathBuf,
    next_id: u64,
}

struct LaunchCleanup {
    child: Option<Child>,
    profile: PathBuf,
}

impl Drop for LaunchCleanup {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = fs::remove_dir_all(&self.profile);
    }
}

fn playwright_cache_dir_names<I, S>(names: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut dirs = names
        .into_iter()
        .filter_map(|name| {
            let name = name.as_ref();
            let (kind, build) = match name.strip_prefix("chromium-") {
                Some(build) => (0, build),
                None => (1, name.strip_prefix("chromium_headless_shell-")?),
            };
            build
                .parse::<u64>()
                .ok()
                .map(|build| (kind, build, name.to_owned()))
        })
        .collect::<Vec<_>>();
    dirs.sort_unstable_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.0.cmp(&b.0))
            .then_with(|| a.2.cmp(&b.2))
    });
    dirs.into_iter()
        .map(|(_, _, name)| name.to_owned())
        .collect()
}

fn playwright_candidates(cache_dir: PathBuf, entries: Vec<PathBuf>) -> Vec<PathBuf> {
    playwright_cache_dir_names(
        entries
            .iter()
            .filter_map(|entry| entry.file_name().and_then(|name| name.to_str())),
    )
    .into_iter()
    .map(|name| cache_dir.join(name))
    .collect()
}
fn candidates(opt: Option<&Path>) -> Vec<PathBuf> {
    if let Some(p) = opt {
        return vec![p.to_path_buf()];
    }
    let mut v = Vec::new();
    for k in ["RIVE_CHROME", "CHROME_PATH"] {
        if let Ok(p) = env::var(k) {
            v.push(PathBuf::from(p))
        }
    }
    if cfg!(target_os = "macos") {
        v.extend(
            [
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                "/Applications/Chromium.app/Contents/MacOS/Chromium",
            ]
            .iter()
            .map(PathBuf::from),
        );
        if let Some(h) = env::var_os("HOME") {
            let d = PathBuf::from(h).join("Library/Caches/ms-playwright");
            if let Ok(es) = fs::read_dir(&d) {
                let entries = es.flatten().map(|e| e.path()).collect();
                for p in playwright_candidates(d, entries) {
                    v.push(p.join(
                        "chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing",
                    ));
                    v.push(p.join(
                        "chrome-mac-x64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing",
                    ));
                    v.push(p.join("chrome-mac/Chromium.app/Contents/MacOS/Chromium"));
                    v.push(p.join("chrome-headless-shell-mac-arm64/chrome-headless-shell"));
                    v.push(p.join("chrome-headless-shell-mac-x64/chrome-headless-shell"));
                    v.push(p.join("chrome-headless-shell-mac/headless_shell"));
                }
            }
        }
    } else if cfg!(target_os = "linux") {
        for n in ["google-chrome", "chromium", "chromium-browser"] {
            v.push(PathBuf::from(n));
        }
        if let Some(h) = env::var_os("HOME") {
            let d = PathBuf::from(h).join(".cache/ms-playwright");
            if let Ok(es) = fs::read_dir(&d) {
                let entries = es.flatten().map(|e| e.path()).collect();
                for p in playwright_candidates(d, entries) {
                    v.push(p.join("chrome-linux64/chrome"));
                    v.push(p.join("chrome-linux-arm64/chrome"));
                    v.push(p.join("chrome-linux/chrome"));
                    v.push(p.join("chrome-headless-shell-linux64/chrome-headless-shell"));
                    v.push(p.join("chrome-headless-shell-linux-arm64/chrome-headless-shell"));
                }
            }
        }
    } else {
        for p in [
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ] {
            v.push(PathBuf::from(p));
        }
    }
    v
}
pub fn discover(opt: Option<&Path>) -> Result<PathBuf, RenderError> {
    let c = candidates(opt);
    for p in &c {
        if p.components().count() == 1 {
            if which(
                p.to_str().ok_or_else(|| {
                    RenderError::Message("invalid Chromium candidate path".into())
                })?,
            )
            .is_some()
            {
                return Ok(p.clone());
            }
        } else if p.is_file() {
            return Ok(p.clone());
        }
    }
    Err(RenderError::Message(format!(
        "No Chromium executable found. Probed:\n{}\nSet $RIVE_CHROME or pass --browser.",
        c.iter()
            .map(|p| format!("  {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n")
    )))
}
fn which(n: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|p| {
        env::split_paths(&p)
            .map(|x| x.join(n))
            .find(|x| x.is_file())
    })
}
impl Chrome {
    pub fn launch(path: &Path, scale: u32) -> Result<Self, RenderError> {
        let profile = env::temp_dir().join(format!(
            "rive-render-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        fs::create_dir_all(&profile)?;
        let mut cleanup = LaunchCleanup {
            child: None,
            profile: profile.clone(),
        };
        let child = Command::new(path)
            .args([
                "--headless=new",
                "--remote-debugging-port=0",
                "--no-first-run",
                "--no-default-browser-check",
                "--disable-gpu",
                "--hide-scrollbars",
                "--disable-dev-shm-usage",
            ])
            .arg(format!("--force-device-scale-factor={scale}"))
            .arg(format!("--user-data-dir={}", profile.display()))
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;
        cleanup.child = Some(child);
        let stderr = cleanup
            .child
            .as_mut()
            .and_then(|child| child.stderr.take())
            .ok_or_else(|| RenderError::Message("missing chromium stderr".into()))?;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        if let Some(i) = line.find("ws://") {
                            let _ = tx.send(line[i..].trim().to_owned());
                            break;
                        }
                    }
                }
            }
        });
        let ws = rx
            .recv_timeout(Duration::from_secs(30))
            .map_err(|_| RenderError::Message("timed out waiting for DevTools".into()))?;
        let (socket, _) = connect(ws).map_err(|e| RenderError::Message(e.to_string()))?;
        let child = cleanup
            .child
            .take()
            .ok_or_else(|| RenderError::Message("launch cleanup child missing".into()))?;
        let mut c = Self {
            child,
            socket,
            session: String::new(),
            profile,
            next_id: 1,
        };
        let t = c.call("Target.createTarget", json!({"url":"about:blank"}), None)?["targetId"]
            .as_str()
            .ok_or_else(|| RenderError::Message("missing target id".into()))?
            .to_string();
        c.session = c.call(
            "Target.attachToTarget",
            json!({"targetId":t,"flatten":true}),
            None,
        )?["sessionId"]
            .as_str()
            .ok_or_else(|| RenderError::Message("missing session id".into()))?
            .to_string();
        cleanup.profile = PathBuf::new();
        Ok(c)
    }
    pub fn call(&mut self, m: &str, p: Value, s: Option<&str>) -> Result<Value, RenderError> {
        let id = self.next_id;
        self.next_id += 1;
        let mut x = json!({"id":id,"method":m,"params":p});
        if let Some(s) = s {
            x["sessionId"] = json!(s)
        }
        self.socket
            .send(Message::Text(x.to_string()))
            .map_err(|e| RenderError::Message(e.to_string()))?;
        loop {
            let msg = self
                .socket
                .read()
                .map_err(|e| RenderError::Message(e.to_string()))?;
            if let Message::Text(t) = msg {
                let v: Value =
                    serde_json::from_str(&t).map_err(|e| RenderError::Message(e.to_string()))?;
                if v.get("id").and_then(Value::as_u64) == Some(id) {
                    if let Some(e) = v.get("error") {
                        return Err(RenderError::Message(e.to_string()));
                    }
                    return Ok(v["result"].clone());
                }
            }
        }
    }
}
impl Drop for Chrome {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.profile);
    }
}

#[cfg(test)]
mod tests {
    use super::playwright_cache_dir_names;

    #[test]
    fn playwright_cache_names_sort_by_descending_build_and_stable_first() {
        let names = playwright_cache_dir_names([
            "chromium-1200",
            "chromium_headless_shell-1223",
            "chromium-1234",
            "chromium-1228",
            "other",
            "chromium_headless_shell-1234",
        ]);
        assert_eq!(
            names,
            vec![
                "chromium-1234",
                "chromium_headless_shell-1234",
                "chromium-1228",
                "chromium_headless_shell-1223",
                "chromium-1200",
            ]
        );
    }
}
