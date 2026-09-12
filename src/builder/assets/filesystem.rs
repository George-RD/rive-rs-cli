use std::borrow::Cow;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

use super::{AssetRequest, AssetResolver, ResolveError};

const PROJECT_MARKERS: [&str; 3] = ["Cargo.toml", ".git", "package.json"];
const OVERSIZE_SENTINEL_BYTES: u64 = 1;

#[derive(Debug)]
pub struct FilesystemAssets {
    base_dir: Option<PathBuf>,
}

impl FilesystemAssets {
    pub fn new(base_dir: Option<&Path>) -> Self {
        Self {
            base_dir: base_dir.map(Path::to_path_buf),
        }
    }
}

impl AssetResolver for FilesystemAssets {
    fn resolve<'a>(
        &'a self,
        request: AssetRequest<'_>,
        max_bytes: usize,
    ) -> Result<Cow<'a, [u8]>, ResolveError> {
        let base_dir = self
            .base_dir
            .as_deref()
            .ok_or(ResolveError::BaseDirectoryRequired)?;
        let source = Path::new(request.source);
        if source
            .components()
            .any(|component| matches!(component, Component::RootDir | Component::Prefix(_)))
        {
            return Err(ResolveError::SourceMustBeRelative);
        }
        let path = base_dir.join(source).canonicalize().map_err(read_error)?;
        let root = project_root(base_dir)?.canonicalize().map_err(read_error)?;
        if !path.starts_with(root) {
            return Err(ResolveError::OutsideProject);
        }
        if !path.metadata().map_err(read_error)?.is_file() {
            return Err(ResolveError::Unavailable);
        }
        let file = File::open(path).map_err(read_error)?;
        let metadata = file.metadata().map_err(read_error)?;
        if !metadata.is_file() {
            return Err(ResolveError::Unavailable);
        }
        let byte_limit = u64::try_from(max_bytes).unwrap_or(u64::MAX);
        if metadata.len() > byte_limit {
            return Err(ResolveError::TooLarge);
        }
        let mut bytes = Vec::new();
        file.take(byte_limit.saturating_add(OVERSIZE_SENTINEL_BYTES))
            .read_to_end(&mut bytes)
            .map_err(read_error)?;
        if bytes.len() > max_bytes {
            return Err(ResolveError::TooLarge);
        }
        Ok(Cow::Owned(bytes))
    }
}

fn read_error(error: io::Error) -> ResolveError {
    if error.kind() == io::ErrorKind::NotFound {
        ResolveError::Missing
    } else {
        ResolveError::Unavailable
    }
}

fn project_root(base_dir: &Path) -> Result<PathBuf, ResolveError> {
    let start = normalise(&std::path::absolute(base_dir).map_err(read_error)?);
    let mut cursor = start.clone();
    loop {
        for marker in PROJECT_MARKERS {
            if cursor.join(marker).try_exists().map_err(read_error)? {
                return Ok(cursor);
            }
        }
        if !cursor.pop() {
            return Ok(start);
        }
    }
}

fn normalise(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            other => result.push(other),
        }
    }
    result
}
