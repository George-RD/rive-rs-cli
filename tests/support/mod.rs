#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rive_cli::builder::{SceneSpec, build_scene};

static WORK_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn assert_builds(scene: serde_json::Value) {
    let scene: SceneSpec =
        serde_json::from_value(scene).expect("lowered SceneSpec must deserialize");
    build_scene(&scene, None).expect("lowered SceneSpec must pass the canonical builder");
}

pub(crate) struct WorkDir(PathBuf);

impl WorkDir {
    pub(crate) fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "{label}-{}-{}",
            std::process::id(),
            WORK_DIR_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("runtime work directory must be created");
        Self(path)
    }

    pub(crate) fn join(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for WorkDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
