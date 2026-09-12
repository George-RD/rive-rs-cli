use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

use thiserror::Error;

mod filesystem;

pub use filesystem::FilesystemAssets;

const MEBIBYTE: usize = 1024 * 1024;
const DEFAULT_PER_ASSET_BYTES: usize = 16 * MEBIBYTE;
const DEFAULT_TOTAL_ASSET_BYTES: usize = 64 * MEBIBYTE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    Image,
    Font,
}

impl fmt::Display for AssetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Image => "image",
            Self::Font => "font",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AssetRequest<'a> {
    pub name: &'a str,
    pub source: &'a str,
    pub kind: AssetKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssetLimits {
    pub per_asset_bytes: usize,
    pub total_bytes: usize,
}

impl Default for AssetLimits {
    fn default() -> Self {
        Self {
            per_asset_bytes: DEFAULT_PER_ASSET_BYTES,
            total_bytes: DEFAULT_TOTAL_ASSET_BYTES,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ResolveError {
    #[error("source could not be read: it is missing")]
    Missing,
    #[error("source could not be read: it is unavailable")]
    Unavailable,
    #[error(
        "embedding asset files is only supported when generating from a scene file on disk; an explicit base directory is required"
    )]
    BaseDirectoryRequired,
    #[error("asset source must be relative to the scene directory")]
    SourceMustBeRelative,
    #[error("asset source resolves outside the project root")]
    OutsideProject,
    #[error("source exceeds the requested byte limit")]
    TooLarge,
}

pub trait AssetResolver {
    fn resolve<'a>(
        &'a self,
        request: AssetRequest<'_>,
        max_bytes: usize,
    ) -> Result<Cow<'a, [u8]>, ResolveError>;
}

#[derive(Debug, Default)]
pub struct MemoryAssets {
    entries: BTreeMap<String, Vec<u8>>,
}

impl MemoryAssets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        source: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Option<Vec<u8>> {
        self.entries.insert(source.into(), bytes.into())
    }
}

impl AssetResolver for MemoryAssets {
    fn resolve<'a>(
        &'a self,
        request: AssetRequest<'_>,
        max_bytes: usize,
    ) -> Result<Cow<'a, [u8]>, ResolveError> {
        let bytes = self
            .entries
            .get(request.source)
            .ok_or(ResolveError::Missing)?;
        if bytes.len() > max_bytes {
            return Err(ResolveError::TooLarge);
        }
        Ok(Cow::Borrowed(bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AssetErrorReason {
    #[error("{0}")]
    Resolve(ResolveError),
    #[error("source is empty")]
    Empty,
    #[error("expected {expected} bytes, detected {actual} bytes")]
    WrongKind {
        expected: AssetKind,
        actual: AssetKind,
    },
    #[error("source exceeds the per-asset limit of {limit} bytes")]
    PerAssetLimit { limit: usize },
    #[error("source exceeds the total supplied-asset limit of {limit} bytes")]
    TotalAssetLimit { limit: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("asset '{asset_name}' source '{source_key}': {reason}")]
pub struct AssetError {
    pub asset_name: String,
    pub source_key: String,
    pub reason: AssetErrorReason,
}

impl AssetError {
    pub const fn code(&self) -> &'static str {
        match &self.reason {
            AssetErrorReason::Resolve(ResolveError::Missing) => "asset-missing",
            AssetErrorReason::Resolve(ResolveError::Unavailable) => "asset-unavailable",
            AssetErrorReason::Resolve(ResolveError::BaseDirectoryRequired) => {
                "asset-base-directory-required"
            }
            AssetErrorReason::Resolve(ResolveError::SourceMustBeRelative) => {
                "asset-source-not-relative"
            }
            AssetErrorReason::Resolve(ResolveError::OutsideProject) => "asset-outside-project",
            AssetErrorReason::Resolve(ResolveError::TooLarge)
            | AssetErrorReason::PerAssetLimit { .. } => "asset-too-large",
            AssetErrorReason::Empty => "asset-empty",
            AssetErrorReason::WrongKind { .. } => "asset-kind-mismatch",
            AssetErrorReason::TotalAssetLimit { .. } => "asset-total-budget-exceeded",
        }
    }
}

pub struct AssetSession<'a> {
    resolver: &'a dyn AssetResolver,
    limits: AssetLimits,
    total_bytes: usize,
}

impl<'a> AssetSession<'a> {
    pub fn new(resolver: &'a dyn AssetResolver, limits: AssetLimits) -> Self {
        Self {
            resolver,
            limits,
            total_bytes: 0,
        }
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn resolve(&mut self, request: AssetRequest<'_>) -> Result<Vec<u8>, AssetError> {
        let remaining = self.limits.total_bytes - self.total_bytes;
        let max_bytes = self.limits.per_asset_bytes.min(remaining);
        let error = |reason| AssetError {
            asset_name: request.name.to_owned(),
            source_key: request.source.to_owned(),
            reason,
        };
        let too_large = || {
            if remaining < self.limits.per_asset_bytes {
                AssetErrorReason::TotalAssetLimit {
                    limit: self.limits.total_bytes,
                }
            } else {
                AssetErrorReason::PerAssetLimit {
                    limit: self.limits.per_asset_bytes,
                }
            }
        };
        let bytes = self
            .resolver
            .resolve(request, max_bytes)
            .map_err(|failure| {
                error(match failure {
                    ResolveError::TooLarge => too_large(),
                    other => AssetErrorReason::Resolve(other),
                })
            })?;
        if bytes.len() > max_bytes {
            return Err(error(too_large()));
        }
        if bytes.is_empty() {
            return Err(error(AssetErrorReason::Empty));
        }
        if let Some(actual) = detected_kind(&bytes)
            && actual != request.kind
        {
            return Err(error(AssetErrorReason::WrongKind {
                expected: request.kind,
                actual,
            }));
        }
        self.total_bytes += bytes.len();
        Ok(bytes.into_owned())
    }
}

fn detected_kind(bytes: &[u8]) -> Option<AssetKind> {
    const FONT_SIGNATURES: [&[u8]; 7] = [
        b"\x00\x01\x00\x00",
        b"OTTO",
        b"true",
        b"typ1",
        b"ttcf",
        b"wOFF",
        b"wOF2",
    ];
    const IMAGE_SIGNATURES: [&[u8]; 5] = [
        b"\x89PNG\r\n\x1a\n",
        b"\xff\xd8\xff",
        b"GIF87a",
        b"GIF89a",
        b"BM",
    ];
    const RIFF_SIGNATURE: &[u8] = b"RIFF";
    const WEBP_SIGNATURE: &[u8] = b"WEBP";
    const RIFF_FORMAT_OFFSET: usize = 8;
    if FONT_SIGNATURES
        .iter()
        .any(|signature| bytes.starts_with(signature))
    {
        return Some(AssetKind::Font);
    }
    let webp = bytes.starts_with(RIFF_SIGNATURE)
        && bytes
            .get(RIFF_FORMAT_OFFSET..)
            .is_some_and(|format| format.starts_with(WEBP_SIGNATURE));
    if webp
        || IMAGE_SIGNATURES
            .iter()
            .any(|signature| bytes.starts_with(signature))
    {
        return Some(AssetKind::Image);
    }
    None
}
