mod chrome;
mod image;
mod server;

use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{fs, path::PathBuf, thread, time::Duration};
use thiserror::Error;

const RIVE_JS: &[u8] = include_bytes!("../../assets/rive.js");
const RIVE_WASM: &[u8] = include_bytes!("../../assets/rive.wasm");
const HARNESS: &[u8] = include_bytes!("../../assets/render-harness.html");
const HARNESS_SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/render-harness.html");
const HARNESS_OVERRIDE_VAR: &str = "RIVE_HARNESS";

fn harness() -> Vec<u8> {
    let candidate = match std::env::var_os(HARNESS_OVERRIDE_VAR) {
        Some(path) => PathBuf::from(path),
        None if cfg!(debug_assertions) => PathBuf::from(HARNESS_SOURCE),
        None => return HARNESS.to_vec(),
    };
    match fs::read(&candidate) {
        Ok(bytes) => bytes,
        Err(_) => HARNESS.to_vec(),
    }
}

const READY_POLL_ATTEMPTS: u32 = 300;
const READY_POLL_INTERVAL: Duration = Duration::from_millis(100);
const TRANSPARENT_BACKGROUND_CHANNEL: u8 = 0;
const TRANSPARENT_BACKGROUND_ALPHA: f64 = 0.0;
const NON_POSITIVE_FPS: f64 = 0.0;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl RenderError {
    fn message(text: impl Into<String>) -> Self {
        Self::Message(text.into())
    }
}

#[derive(Clone)]
pub struct RenderOptions {
    pub riv: Vec<u8>,
    pub source_path: PathBuf,
    pub output_dir: PathBuf,
    pub frames: Vec<u32>,
    pub fps: f64,
    pub animation: Option<String>,
    pub state_machine: Option<String>,
    pub inputs: Vec<String>,
    pub pointers: Vec<String>,
    pub artboard: Option<String>,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub background: Option<String>,
    pub contact_sheet: bool,
    pub preview: bool,
    pub browser: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderedFrame {
    pub index: u32,
    pub seconds: f64,
    pub filename: String,
    pub distinct_colors: usize,
    pub blank: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<image::CoveragePreview>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderManifest {
    pub source: String,
    pub artboard: String,
    pub animation: Option<String>,
    pub state_machine: Option<String>,
    pub available_animations: Vec<String>,
    pub available_state_machines: Vec<String>,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub fps: f64,
    pub frames: Vec<RenderedFrame>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applied_inputs: Vec<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applied_pointers: Vec<Value>,
    pub contact_sheet: Option<String>,
}

pub fn parse_frame_spec(spec: &str) -> Result<Vec<u32>, RenderError> {
    if spec.trim().is_empty() {
        return Err(RenderError::message("frame specification is empty"));
    }
    let mut frames = Vec::new();
    for part in spec.split(',') {
        let part = part.trim();
        if part.is_empty() {
            return Err(RenderError::message(
                "frame list contains an empty item; use a form like 0,15,30",
            ));
        }
        match split_range(part)? {
            Some((start, end, step)) => {
                let mut current = start;
                while current < end {
                    frames.push(current);
                    let Some(next) = current.checked_add(step) else {
                        break;
                    };
                    if next >= end {
                        break;
                    }
                    current = next;
                }
            }
            None => frames.push(parse_index(part, "frame index")?),
        }
    }
    if frames.is_empty() {
        return Err(RenderError::message(
            "frame specification selects no frames",
        ));
    }
    Ok(frames)
}

fn split_range(part: &str) -> Result<Option<(u32, u32, u32)>, RenderError> {
    let (range, step) = match part.split_once(':') {
        Some((range, step)) => (range, parse_index(step, "range step")?),
        None => (part, 1),
    };
    let Some((start, end)) = range.split_once("..") else {
        if part.contains(':') {
            return Err(RenderError::message(format!(
                "'{part}' has a step but no range; use start..end:step"
            )));
        }
        return Ok(None);
    };
    if step == 0 {
        return Err(RenderError::message("range step must be greater than zero"));
    }
    let start = parse_index(start, "range start")?;
    let end = parse_index(end, "range end")?;
    if end < start {
        return Err(RenderError::message(format!(
            "range '{range}' is inverted; start must not exceed end"
        )));
    }
    Ok(Some((start, end, step)))
}

fn parse_index(text: &str, label: &str) -> Result<u32, RenderError> {
    text.trim().parse().map_err(|_| {
        RenderError::message(format!(
            "invalid {label}: '{text}' is not a non-negative whole number"
        ))
    })
}

pub fn render(options: &RenderOptions) -> Result<RenderManifest, RenderError> {
    if options.width == 0 || options.height == 0 || options.scale == 0 {
        return Err(RenderError::message(
            "width, height and scale must all be greater than zero",
        ));
    }
    validate_fps(options.fps)?;
    fs::create_dir_all(&options.output_dir)?;

    let background = options
        .background
        .as_deref()
        .map(parse_background)
        .transpose()?;
    let server = server::AssetServer::start(harness(), RIVE_JS, RIVE_WASM, options.riv.clone())?;
    let browser_path = chrome::discover(options.browser.as_deref())?;
    let mut browser = chrome::Chrome::launch(&browser_path, options.scale)?;
    let session = browser.session.clone();

    browser.call("Page.enable", json!({}), Some(&session))?;
    browser.call("Runtime.enable", json!({}), Some(&session))?;
    browser.call(
        "Emulation.setDeviceMetricsOverride",
        json!({
            "width": options.width,
            "height": options.height,
            "deviceScaleFactor": options.scale,
            "mobile": false,
        }),
        Some(&session),
    )?;
    browser.call(
        "Page.navigate",
        json!({ "url": server.url }),
        Some(&session),
    )?;
    wait_for_document(&mut browser, &session)?;

    let scene = load_scene(&mut browser, &session, options, background.as_deref())?;
    set_capture_background(&mut browser, &session, background.is_none())?;

    let mut frames = Vec::new();
    let mut written = Vec::new();
    for &index in &options.frames {
        let seconds = frame_seconds(index, options.fps)?;
        let seek = browser.call(
            "Runtime.evaluate",
            json!({
                "expression": format!("window.riveSeek({seconds})"),
                "awaitPromise": true,
            }),
            Some(&session),
        )?;
        if let Some(details) = seek.get("exceptionDetails") {
            return Err(RenderError::message(format!(
                "the Rive runtime could not seek to frame {index}: {}",
                exception_text(details)
            )));
        }
        let shot = browser.call(
            "Page.captureScreenshot",
            json!({
                "format": "png",
                "captureBeyondViewport": false,
                "clip": {
                    "x": 0,
                    "y": 0,
                    "width": options.width,
                    "height": options.height,
                    "scale": 1,
                },
            }),
            Some(&session),
        )?;
        let encoded = shot["data"]
            .as_str()
            .ok_or_else(|| RenderError::message("browser returned no screenshot data"))?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|error| {
                RenderError::message(format!("could not decode screenshot: {error}"))
            })?;
        let filename = format!("frame_{index:05}.png");
        let path = options.output_dir.join(&filename);
        fs::write(&path, bytes)?;
        let analysis = image::analyze(&path)?;
        let preview = options.preview.then(|| image::coverage_preview(&analysis));
        if let Some(preview) = &preview {
            eprintln!("frame {index}:\n{}", preview.text);
        }
        frames.push(RenderedFrame {
            index,
            seconds,
            filename,
            distinct_colors: analysis.distinct_colors,
            blank: analysis.blank,
            preview,
        });
        written.push(path);
    }

    let contact_sheet = if options.contact_sheet {
        let path = options.output_dir.join("contact_sheet.png");
        image::contact_sheet(&written, &path)?;
        Some(path.to_string_lossy().into_owned())
    } else {
        None
    };
    if options.preview {
        let mut preview_text = String::from("Rive render coverage preview\n");
        for frame in &frames {
            if let Some(preview) = &frame.preview {
                preview_text.push_str(&format!(
                    "frame {} @ {:.3}s ({})\n{}",
                    frame.index, frame.seconds, frame.filename, preview.text
                ));
            }
        }
        fs::write(options.output_dir.join("preview.txt"), preview_text)?;
    }

    let manifest = RenderManifest {
        source: options.source_path.to_string_lossy().into_owned(),
        artboard: scene.artboard,
        animation: scene.selected_animation,
        state_machine: scene.selected_state_machine,
        available_animations: scene.animations,
        available_state_machines: scene.state_machines,
        width: options.width,
        height: options.height,
        scale: options.scale,
        fps: options.fps,
        frames,
        applied_inputs: scene.applied_inputs,
        applied_pointers: scene.applied_pointers,
        contact_sheet,
    };
    fs::write(
        options.output_dir.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    drop(server);
    Ok(manifest)
}

struct LoadedScene {
    artboard: String,
    animations: Vec<String>,
    state_machines: Vec<String>,
    selected_animation: Option<String>,
    selected_state_machine: Option<String>,
    applied_inputs: Vec<Value>,
    applied_pointers: Vec<Value>,
}

fn wait_for_document(browser: &mut chrome::Chrome, session: &str) -> Result<(), RenderError> {
    const PROBE: &str = "JSON.stringify({ready: document.readyState, url: location.href, runtime: typeof window.rive, harness: typeof window.riveLoad, error: window.__riveHarnessError || null})";
    let mut last = String::from("no response from the browser");
    for _ in 0..READY_POLL_ATTEMPTS {
        let state = browser.call(
            "Runtime.evaluate",
            json!({ "expression": PROBE, "returnByValue": true }),
            Some(session),
        )?;
        if let Some(text) = state["result"]["value"].as_str() {
            last = text.to_string();
            let ready = text.contains("\"ready\":\"complete\"");
            let harness = text.contains("\"harness\":\"function\"");
            if ready && harness {
                return Ok(());
            }
        }
        thread::sleep(READY_POLL_INTERVAL);
    }
    Err(RenderError::message(format!(
        "timed out waiting for the render harness to load; last browser state: {last}"
    )))
}

fn validate_fps(fps: f64) -> Result<(), RenderError> {
    if !fps.is_finite() || fps <= NON_POSITIVE_FPS {
        return Err(RenderError::message(
            "fps must be a finite number greater than zero",
        ));
    }
    Ok(())
}

fn frame_seconds(index: u32, fps: f64) -> Result<f64, RenderError> {
    let seconds = f64::from(index) / fps;
    if !seconds.is_finite() {
        return Err(RenderError::message(
            "frame timestamp must be a finite number",
        ));
    }
    Ok(seconds)
}

fn set_capture_background(
    browser: &mut chrome::Chrome,
    session: &str,
    transparent: bool,
) -> Result<(), RenderError> {
    let parameters = if transparent {
        json!({
            "color": {
                "r": TRANSPARENT_BACKGROUND_CHANNEL,
                "g": TRANSPARENT_BACKGROUND_CHANNEL,
                "b": TRANSPARENT_BACKGROUND_CHANNEL,
                "a": TRANSPARENT_BACKGROUND_ALPHA,
            }
        })
    } else {
        json!({})
    };
    browser.call(
        "Emulation.setDefaultBackgroundColorOverride",
        parameters,
        Some(session),
    )?;
    Ok(())
}

fn split_frame_suffix(entry: &str, body: &str) -> Result<(String, Option<u32>), RenderError> {
    let Some((head, frame)) = body.rsplit_once('@') else {
        return Ok((body.to_string(), None));
    };
    let frame: u32 = frame.trim().parse().map_err(|_| {
        RenderError::message(format!(
            "invalid frame suffix in '{entry}': '{frame}' is not a non-negative frame index"
        ))
    })?;
    Ok((head.to_string(), Some(frame)))
}

fn parse_input(entry: &str) -> Result<Value, RenderError> {
    let (name, raw) = entry.split_once('=').ok_or_else(|| {
        RenderError::message(format!(
            "invalid --input '{entry}': expected NAME=VALUE[@FRAME], e.g. isHovered=true or press=trigger@30"
        ))
    })?;
    let name = name.trim();
    let (raw, frame) = split_frame_suffix(entry, raw.trim())?;
    let raw = raw.trim();
    if name.is_empty() {
        return Err(RenderError::message(format!(
            "invalid --input '{entry}': the input name is empty"
        )));
    }
    let mut value = match raw {
        "true" => json!({ "name": name, "kind": "bool", "value": true }),
        "false" => json!({ "name": name, "kind": "bool", "value": false }),
        "trigger" => json!({ "name": name, "kind": "trigger" }),
        other => {
            let number: f64 = other.parse().map_err(|_| {
                RenderError::message(format!(
                    "invalid --input '{entry}': '{other}' is not true, false, trigger, or a number"
                ))
            })?;
            if !number.is_finite() {
                return Err(RenderError::message(format!(
                    "invalid --input '{entry}': '{other}' is not a finite number"
                )));
            }
            json!({ "name": name, "kind": "number", "value": number })
        }
    };
    value["frame"] = match frame {
        Some(frame) => json!(frame),
        None => Value::Null,
    };
    Ok(value)
}

const POINTER_EVENTS: [&str; 5] = ["down", "up", "move", "enter", "exit"];

fn parse_pointer(entry: &str) -> Result<Value, RenderError> {
    let (event, rest) = entry.split_once(':').ok_or_else(|| {
        RenderError::message(format!(
            "invalid --pointer '{entry}': expected EVENT:X,Y@FRAME, e.g. down:120,90@10"
        ))
    })?;
    let event = event.trim();
    if !POINTER_EVENTS.contains(&event) {
        return Err(RenderError::message(format!(
            "invalid --pointer '{entry}': '{event}' is not one of {}",
            POINTER_EVENTS.join(", ")
        )));
    }
    let (coords, frame) = split_frame_suffix(entry, rest.trim())?;
    let Some(frame) = frame else {
        return Err(RenderError::message(format!(
            "invalid --pointer '{entry}': a frame is required, e.g. {event}:120,90@10"
        )));
    };
    let (x, y) = coords.split_once(',').ok_or_else(|| {
        RenderError::message(format!(
            "invalid --pointer '{entry}': expected artboard coordinates X,Y"
        ))
    })?;
    let parse_coord = |raw: &str, axis: &str| -> Result<f64, RenderError> {
        let value: f64 = raw.trim().parse().map_err(|_| {
            RenderError::message(format!(
                "invalid --pointer '{entry}': {axis} coordinate '{}' is not a number",
                raw.trim()
            ))
        })?;
        if !value.is_finite() {
            return Err(RenderError::message(format!(
                "invalid --pointer '{entry}': {axis} coordinate is not finite"
            )));
        }
        Ok(value)
    };
    Ok(json!({
        "event": event,
        "x": parse_coord(x, "x")?,
        "y": parse_coord(y, "y")?,
        "frame": frame,
    }))
}

fn load_scene(
    browser: &mut chrome::Chrome,
    session: &str,
    options: &RenderOptions,
    background: Option<&str>,
) -> Result<LoadedScene, RenderError> {
    let inputs = options
        .inputs
        .iter()
        .map(|entry| parse_input(entry))
        .collect::<Result<Vec<_>, _>>()?;
    let pointers = options
        .pointers
        .iter()
        .map(|entry| parse_pointer(entry))
        .collect::<Result<Vec<_>, _>>()?;
    if !inputs.is_empty() && options.state_machine.is_none() {
        return Err(RenderError::message(
            "--input only applies when --state-machine is given",
        ));
    }
    if !pointers.is_empty() && options.state_machine.is_none() {
        return Err(RenderError::message(
            "--pointer only applies when --state-machine is given",
        ));
    }
    let request = json!({
        "width": options.width,
        "height": options.height,
        "scale": options.scale,
        "fps": options.fps,
        "artboard": options.artboard,
        "animation": options.animation,
        "stateMachine": options.state_machine,
        "inputs": inputs,
        "pointers": pointers,
        "background": background,
    });
    let evaluated = browser.call(
        "Runtime.evaluate",
        json!({
            "expression": format!("window.riveLoad({request})"),
            "awaitPromise": true,
            "returnByValue": true,
        }),
        Some(session),
    )?;
    if let Some(details) = evaluated.get("exceptionDetails") {
        return Err(RenderError::message(format!(
            "the Rive runtime could not load this file: {}",
            exception_text(details)
        )));
    }
    let value = evaluated
        .get("result")
        .and_then(|result| result.get("value"))
        .ok_or_else(|| RenderError::message("render harness returned no scene description"))?;
    if let Some(error) = value.get("error").and_then(Value::as_str) {
        return Err(RenderError::message(error));
    }
    let animations = string_list(value.get("animations"));
    let state_machines = string_list(value.get("stateMachines"));
    let selected = value
        .get("selected")
        .and_then(Value::as_str)
        .map(str::to_string);
    let is_state_machine = value.get("mode").and_then(Value::as_str) == Some("stateMachine");
    Ok(LoadedScene {
        artboard: value
            .get("artboard")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        animations,
        state_machines,
        selected_animation: if is_state_machine {
            None
        } else {
            selected.clone()
        },
        selected_state_machine: if is_state_machine { selected } else { None },
        applied_inputs: inputs,
        applied_pointers: pointers,
    })
}

fn exception_text(details: &Value) -> String {
    details
        .get("exception")
        .and_then(|exception| exception.get("description"))
        .or_else(|| details.get("text"))
        .and_then(Value::as_str)
        .unwrap_or("unknown error")
        .to_string()
}

fn string_list(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn parse_background(value: &str) -> Result<String, RenderError> {
    let digits = value.strip_prefix('#').ok_or_else(|| {
        RenderError::message(format!(
            "invalid background '{value}': expected #RRGGBB or #RRGGBBAA"
        ))
    })?;
    let valid_length = digits.len() == 6 || digits.len() == 8;
    if !valid_length || !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(RenderError::message(format!(
            "invalid background '{value}': expected #RRGGBB or #RRGGBBAA"
        )));
    }
    Ok(value.to_string())
}

pub fn render_manifest_text(manifest: &RenderManifest) -> String {
    let scene = match (&manifest.animation, &manifest.state_machine) {
        (_, Some(machine)) => format!("state machine '{machine}'"),
        (Some(animation), _) => format!("animation '{animation}'"),
        _ => "static artboard".to_string(),
    };
    let mut text = format!(
        "artboard '{}' | {} | {}x{} @{}x | {} fps\n",
        manifest.artboard, scene, manifest.width, manifest.height, manifest.scale, manifest.fps
    );
    text.push_str("  frame   seconds  file                 colors\n");
    for frame in &manifest.frames {
        text.push_str(&format!(
            "  {:>5}  {:>8.3}  {:<20} {:>6}{}\n",
            frame.index,
            frame.seconds,
            frame.filename,
            frame.distinct_colors,
            if frame.blank { "  BLANK" } else { "" }
        ));
    }
    for input in &manifest.applied_inputs {
        let name = input["name"].as_str().unwrap_or("?");
        let when = match input["frame"].as_u64() {
            Some(frame) => format!("frame {frame}"),
            None => "before playback".to_string(),
        };
        let value = match input["kind"].as_str() {
            Some("trigger") => "trigger".to_string(),
            _ => input["value"].to_string(),
        };
        text.push_str(&format!("  input {name} = {value} @ {when}\n"));
    }
    for pointer in &manifest.applied_pointers {
        text.push_str(&format!(
            "  pointer {} at {},{} @ frame {}\n",
            pointer["event"].as_str().unwrap_or("?"),
            pointer["x"],
            pointer["y"],
            pointer["frame"]
        ));
    }
    if let Some(sheet) = &manifest.contact_sheet {
        text.push_str(&format!("  contact sheet: {sheet}\n"));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_frame_lists_ranges_and_steps() {
        assert_eq!(parse_frame_spec("0,15,30").unwrap(), vec![0, 15, 30]);
        assert_eq!(parse_frame_spec("30").unwrap(), vec![30]);
        assert_eq!(parse_frame_spec("0..5:2").unwrap(), vec![0, 2, 4]);
        assert_eq!(parse_frame_spec("0..3").unwrap(), vec![0, 1, 2]);
        assert_eq!(parse_frame_spec("0..2,10").unwrap(), vec![0, 1, 10]);
        assert_eq!(parse_frame_spec(" 0 , 4 ").unwrap(), vec![0, 4]);
    }

    #[test]
    fn rejects_malformed_frame_specs() {
        assert!(parse_frame_spec("").is_err());
        assert!(parse_frame_spec("0,,3").is_err());
        assert!(parse_frame_spec("0..3:0").is_err());
        assert!(parse_frame_spec("3..0").is_err());
        assert!(parse_frame_spec("-1").is_err());
        assert!(parse_frame_spec("abc").is_err());
        assert!(parse_frame_spec("5:2").is_err());
        assert!(parse_frame_spec("0..3:-1").is_err());
    }

    #[test]
    fn range_end_is_exclusive() {
        assert_eq!(parse_frame_spec("0..1").unwrap(), vec![0]);
        assert!(parse_frame_spec("2..2").is_err());
    }

    #[test]
    fn range_step_near_u32_max_terminates() {
        assert_eq!(
            parse_frame_spec("4294967294..4294967295:2").unwrap(),
            vec![u32::MAX - 1]
        );
    }

    #[test]
    fn rejects_non_finite_and_degenerate_fps() {
        assert!(validate_fps(f64::NAN).is_err());
        assert!(validate_fps(f64::MIN_POSITIVE).is_ok());
        assert!(frame_seconds(u32::MAX, f64::MIN_POSITIVE).is_err());
    }

    #[test]
    fn parses_state_machine_inputs() {
        assert_eq!(parse_input("isOn=true").unwrap()["kind"], "bool");
        assert_eq!(parse_input("isOn=true").unwrap()["value"], true);
        assert_eq!(parse_input("isOn=false").unwrap()["value"], false);
        assert_eq!(parse_input("press=trigger").unwrap()["kind"], "trigger");
        assert_eq!(parse_input("level=0.25").unwrap()["value"], 0.25);
        assert_eq!(parse_input(" level = 3 ").unwrap()["name"], "level");
    }

    #[test]
    fn rejects_malformed_state_machine_inputs() {
        assert!(parse_input("noEquals").is_err());
        assert!(parse_input("=true").is_err());
        assert!(parse_input("bad=maybe").is_err());
        assert!(parse_input("bad=NaN").is_err());
        assert!(parse_input("bad=inf").is_err());
        assert!(parse_input("bad=-inf").is_err());
    }

    #[test]
    fn parses_scheduled_state_machine_inputs() {
        assert_eq!(parse_input("isOn=true").unwrap()["frame"], Value::Null);
        let scheduled = parse_input("press=trigger@30").unwrap();
        assert_eq!(scheduled["kind"], "trigger");
        assert_eq!(scheduled["frame"], 30);
        assert_eq!(parse_input("level=0.5@0").unwrap()["frame"], 0);
    }

    #[test]
    fn rejects_malformed_input_frame_suffixes() {
        assert!(parse_input("x=true@").is_err());
        assert!(parse_input("x=true@-1").is_err());
        assert!(parse_input("x=true@abc").is_err());
    }

    #[test]
    fn parses_pointer_events() {
        let pointer = parse_pointer("down:120,90@10").unwrap();
        assert_eq!(pointer["event"], "down");
        assert_eq!(pointer["x"], 120.0);
        assert_eq!(pointer["y"], 90.0);
        assert_eq!(pointer["frame"], 10);
        assert_eq!(parse_pointer("move:-4.5,0.25@3").unwrap()["x"], -4.5);
        for event in POINTER_EVENTS {
            assert!(parse_pointer(&format!("{event}:1,2@0")).is_ok());
        }
    }

    #[test]
    fn rejects_malformed_pointer_events() {
        assert!(parse_pointer("down 120,90@10").is_err());
        assert!(parse_pointer("wiggle:1,2@0").is_err());
        assert!(parse_pointer("down:1,2").is_err());
        assert!(parse_pointer("down:1@0").is_err());
        assert!(parse_pointer("down:a,2@0").is_err());
        assert!(parse_pointer("down:1,2@x").is_err());
    }

    #[test]
    fn accepts_valid_backgrounds_and_rejects_others() {
        assert!(parse_background("#FF0000").is_ok());
        assert!(parse_background("#FF0000CC").is_ok());
        assert!(parse_background("FF0000").is_err());
        assert!(parse_background("#FFF").is_err());
        assert!(parse_background("#GGGGGG").is_err());
    }
}
