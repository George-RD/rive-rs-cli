use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const FILE_ID: u64 = 0x1020_3040;

fn json_files(directory: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            json_files(&path, files)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            files.push(path);
        }
    }
    Ok(())
}

fn digest_scene(path: &Path) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(path)?;
    let document: Value = match serde_json::from_str(&input) {
        Ok(document) => document,
        Err(_) => return Ok(None),
    };
    let scene: SceneSpec = if document.get("authoring_format_version").is_some() {
        match lower_authoring_json(&input) {
            Ok(lowered) => serde_json::from_value(lowered.scene)?,
            Err(_) => return Ok(None),
        }
    } else if document.get("scene_format_version").is_some() {
        match serde_json::from_value(document) {
            Ok(scene) => scene,
            Err(_) => return Ok(None),
        }
    } else {
        return Ok(None);
    };
    Ok(compile_scene(&scene, path.parent(), FILE_ID)
        .ok()
        .map(|bytes| format!("{:x}", Sha256::digest(bytes))))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    for directory in ["tests/fixtures", "examples/authoring", "showcase"] {
        json_files(&root.join(directory), &mut files)?;
    }
    files.sort();
    let mut digests = BTreeMap::new();
    let mut skipped = Vec::new();
    for path in files {
        let relative = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        match digest_scene(&path)? {
            Some(digest) => {
                digests.insert(relative, digest);
            }
            None => skipped.push(relative),
        }
    }
    if digests.is_empty() {
        return Err("no valid scene fixtures were compiled".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "file_id": FILE_ID,
            "digests": digests,
            "skipped": skipped
        }))?
    );
    Ok(())
}
