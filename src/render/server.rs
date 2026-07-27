use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const ACCEPT_IDLE: Duration = Duration::from_millis(5);
const SOCKET_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_REQUEST_BYTES: usize = 16 * 1024;

pub struct AssetServer {
    pub url: String,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

struct Assets {
    html: &'static [u8],
    js: &'static [u8],
    wasm: &'static [u8],
    scene: Vec<u8>,
}

impl AssetServer {
    pub fn start(
        html: &'static [u8],
        js: &'static [u8],
        wasm: &'static [u8],
        scene: Vec<u8>,
    ) -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        listener.set_nonblocking(true)?;
        let url = format!("http://{}/", listener.local_addr()?);
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let assets = Arc::new(Assets {
            html,
            js,
            wasm,
            scene,
        });
        let thread = thread::spawn(move || {
            let mut workers: Vec<JoinHandle<()>> = Vec::new();
            while !flag.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let assets = assets.clone();
                        workers.push(thread::spawn(move || serve(stream, &assets)));
                        workers.retain(|worker| !worker.is_finished());
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(ACCEPT_IDLE);
                    }
                    Err(_) => break,
                }
            }
            for worker in workers {
                let _ = worker.join();
            }
        });
        Ok(Self {
            url,
            stop,
            thread: Some(thread),
        })
    }
}

impl Drop for AssetServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn serve(mut stream: TcpStream, assets: &Assets) {
    if stream.set_nonblocking(false).is_err() {
        return;
    }
    let _ = stream.set_read_timeout(Some(SOCKET_TIMEOUT));
    let _ = stream.set_write_timeout(Some(SOCKET_TIMEOUT));

    let Some(request) = read_request(&mut stream) else {
        return;
    };
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("");
    let route = path.split('?').next().unwrap_or(path);
    let (status, content_type, body): (&str, &str, &[u8]) = match route {
        "/" | "/index.html" => ("200 OK", "text/html; charset=utf-8", assets.html),
        "/rive.js" => ("200 OK", "application/javascript", assets.js),
        "/rive.wasm" => ("200 OK", "application/wasm", assets.wasm),
        "/scene.riv" => ("200 OK", "application/octet-stream", &assets.scene),
        _ => ("404 Not Found", "text/plain; charset=utf-8", b"not found"),
    };
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    if stream.write_all(header.as_bytes()).is_err() {
        return;
    }
    let _ = stream.write_all(body);
    let _ = stream.flush();
}

fn read_request(stream: &mut TcpStream) -> Option<String> {
    let mut collected: Vec<u8> = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        let read = stream.read(&mut chunk).ok()?;
        if read == 0 {
            break;
        }
        collected.extend_from_slice(&chunk[..read]);
        if collected.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
        if collected.len() > MAX_REQUEST_BYTES {
            return None;
        }
    }
    if collected.is_empty() {
        return None;
    }
    Some(String::from_utf8_lossy(&collected).into_owned())
}
