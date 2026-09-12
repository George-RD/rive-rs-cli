use std::borrow::Cow;
use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rive_cli::compile::assets::{
    AssetErrorReason, AssetKind, AssetLimits, AssetRequest, AssetResolver, AssetSession,
    FilesystemAssets, MemoryAssets, ResolveError,
};

const FONT: &[u8] = include_bytes!("../assets/fonts/Inter-Bold-Subset.ttf");
const IMAGE: &[u8] = include_bytes!("../assets/textures/aurora.png");

fn request<'a>(name: &'a str, source: &'a str, kind: AssetKind) -> AssetRequest<'a> {
    AssetRequest { name, source, kind }
}

fn memory_assets() -> MemoryAssets {
    let mut assets = MemoryAssets::new();
    assets.insert("font-key", FONT);
    assets.insert("image-key", IMAGE);
    assets
}

#[test]
fn memory_resolver_borrows_bytes_without_consuming_them() {
    let assets = memory_assets();
    for _ in 0..2 {
        let bytes = assets
            .resolve(request("font", "font-key", AssetKind::Font), FONT.len())
            .expect("resolve embedded font");
        assert!(matches!(bytes, Cow::Borrowed(_)));
        assert_eq!(&*bytes, FONT);
    }
}

#[test]
fn memory_session_returns_owned_image_and_font_bytes_repeatably() {
    let assets = memory_assets();
    for _ in 0..2 {
        let mut session = AssetSession::new(&assets, AssetLimits::default());
        let mut font = session
            .resolve(request("font", "font-key", AssetKind::Font))
            .expect("font");
        assert_eq!(font, FONT);
        font.fill(0);
        assert_eq!(
            session
                .resolve(request("image", "image-key", AssetKind::Image))
                .expect("image"),
            IMAGE
        );
        assert_eq!(session.total_bytes(), FONT.len() + IMAGE.len());
    }
}

#[test]
fn memory_keys_are_literal_and_missing_bytes_do_not_fall_back_to_disk() {
    let assets = memory_assets();
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    for key in ["./font-key", "assets/fonts/Inter-Bold-Subset.ttf"] {
        let error = session
            .resolve(request("logical-font", key, AssetKind::Font))
            .expect_err("no path lookup or filesystem fallback");
        assert_eq!(error.code(), "asset-missing");
        assert_eq!(error.asset_name, "logical-font");
        assert_eq!(error.source_key, key);
    }
    assert_eq!(session.total_bytes(), 0);
}

#[test]
fn empty_bytes_are_distinct_from_missing_bytes() {
    let mut assets = MemoryAssets::new();
    assets.insert("empty", Vec::new());
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    let error = session
        .resolve(request("picture", "empty", AssetKind::Image))
        .expect_err("empty payload");
    assert_eq!(error.code(), "asset-empty");
    assert_eq!(session.total_bytes(), 0);
}

#[test]
fn known_image_and_font_signatures_reject_cross_kind_use() {
    let assets = memory_assets();
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    for (key, expected, actual) in [
        ("font-key", AssetKind::Image, AssetKind::Font),
        ("image-key", AssetKind::Font, AssetKind::Image),
    ] {
        let error = session
            .resolve(request("wrong-kind", key, expected))
            .expect_err("detect kind mismatch");
        assert_eq!(error.code(), "asset-kind-mismatch");
        assert_eq!(error.reason, AssetErrorReason::WrongKind { expected, actual });
    }
    assert_eq!(session.total_bytes(), 0);
}

#[test]
fn unrecognized_signatures_are_not_claimed_to_be_invalid_formats() {
    let mut assets = MemoryAssets::new();
    assets.insert("opaque", b"opaque".as_slice());
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    assert_eq!(
        session
            .resolve(request("opaque", "opaque", AssetKind::Image))
            .expect("unknown signatures retain compatibility"),
        b"opaque"
    );
}

#[test]
fn exact_per_asset_and_total_limits_are_inclusive() {
    let assets = memory_assets();
    let mut session = AssetSession::new(
        &assets,
        AssetLimits { per_asset_bytes: FONT.len(), total_bytes: FONT.len() },
    );
    assert_eq!(
        session
            .resolve(request("font", "font-key", AssetKind::Font))
            .expect("exact limit"),
        FONT
    );
    assert_eq!(session.total_bytes(), FONT.len());
}

#[test]
fn per_asset_limit_is_reported_with_the_logical_asset() {
    let assets = memory_assets();
    let mut session = AssetSession::new(
        &assets,
        AssetLimits { per_asset_bytes: FONT.len() - 1, total_bytes: usize::MAX },
    );
    let error = session
        .resolve(request("font", "font-key", AssetKind::Font))
        .expect_err("per-asset limit");
    assert_eq!(error.code(), "asset-too-large");
    assert_eq!(error.asset_name, "font");
    assert_eq!(session.total_bytes(), 0);
}

#[test]
fn total_budget_counts_repeated_emitted_assets_not_unique_keys() {
    let assets = memory_assets();
    let mut session = AssetSession::new(
        &assets,
        AssetLimits { per_asset_bytes: FONT.len(), total_bytes: FONT.len() * 2 - 1 },
    );
    session.resolve(request("first", "font-key", AssetKind::Font)).expect("first font");
    let error = session
        .resolve(request("second", "font-key", AssetKind::Font))
        .expect_err("second emitted contents exceeds total");
    assert_eq!(error.code(), "asset-total-budget-exceeded");
    assert_eq!(error.asset_name, "second");
    assert_eq!(session.total_bytes(), FONT.len());
}

#[test]
fn zero_budget_does_not_turn_nonempty_data_into_an_empty_asset() {
    let assets = memory_assets();
    let mut session = AssetSession::new(
        &assets,
        AssetLimits { per_asset_bytes: FONT.len(), total_bytes: 0 },
    );
    assert_eq!(
        session
            .resolve(request("font", "font-key", AssetKind::Font))
            .expect_err("zero total budget")
            .code(),
        "asset-total-budget-exceeded"
    );
}

struct UnboundedResolver {
    requested_limit: Cell<usize>,
}

impl AssetResolver for UnboundedResolver {
    fn resolve<'a>(
        &'a self,
        _request: AssetRequest<'_>,
        max_bytes: usize,
    ) -> Result<Cow<'a, [u8]>, ResolveError> {
        self.requested_limit.set(max_bytes);
        Ok(Cow::Borrowed(FONT))
    }
}

#[test]
fn session_enforces_limits_even_when_a_custom_resolver_ignores_them() {
    let assets = UnboundedResolver { requested_limit: Cell::new(usize::MAX) };
    let mut session = AssetSession::new(
        &assets,
        AssetLimits { per_asset_bytes: FONT.len(), total_bytes: FONT.len() - 1 },
    );
    let error = session
        .resolve(request("font", "font-key", AssetKind::Font))
        .expect_err("independent enforcement");
    assert_eq!(assets.requested_limit.get(), FONT.len() - 1);
    assert_eq!(error.code(), "asset-total-budget-exceeded");
    assert_eq!(session.total_bytes(), 0);
}

struct TestProject(PathBuf);

impl TestProject {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        loop {
            let path = std::env::temp_dir().join(format!(
                "rive-257-assets-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("create test directory: {error}"),
            }
        }
    }

    fn root(&self) -> &Path {
        &self.0
    }

    fn prepare(&self) -> PathBuf {
        std::fs::write(self.root().join("Cargo.toml"), "").expect("project marker");
        std::fs::write(self.root().join("font.ttf"), FONT).expect("font file");
        let base = self.root().join("scenes");
        std::fs::create_dir(&base).expect("scene directory");
        base
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn filesystem_adapter_allows_project_relative_parent_paths() {
    let project = TestProject::new();
    let base = project.prepare();
    let assets = FilesystemAssets::new(Some(&base));
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    assert_eq!(
        session
            .resolve(request("font", "../font.ttf", AssetKind::Font))
            .expect("parent source within project"),
        FONT
    );
}

#[test]
fn filesystem_adapter_requires_an_explicit_base_directory() {
    let assets = FilesystemAssets::new(None);
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    assert_eq!(
        session
            .resolve(request("font", "font.ttf", AssetKind::Font))
            .expect_err("no ambient current directory fallback")
            .code(),
        "asset-base-directory-required"
    );
}

#[test]
fn filesystem_adapter_rejects_absolute_sources() {
    let project = TestProject::new();
    let base = project.prepare();
    let assets = FilesystemAssets::new(Some(&base));
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    let absolute = project.root().join("font.ttf");
    assert_eq!(
        session
            .resolve(request("font", absolute.to_str().expect("test path"), AssetKind::Font))
            .expect_err("absolute source")
            .code(),
        "asset-source-not-relative"
    );
}

#[test]
fn filesystem_adapter_distinguishes_missing_empty_and_directory_sources() {
    let project = TestProject::new();
    let base = project.prepare();
    std::fs::write(base.join("empty"), "").expect("empty file");
    let assets = FilesystemAssets::new(Some(&base));
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    for (source, code) in [
        ("missing", "asset-missing"),
        ("empty", "asset-empty"),
        (".", "asset-unavailable"),
    ] {
        assert_eq!(
            session
                .resolve(request("font", source, AssetKind::Font))
                .expect_err("invalid source")
                .code(),
            code
        );
    }
}

#[test]
fn filesystem_adapter_bounds_reads_before_returning_payloads() {
    let project = TestProject::new();
    let base = project.prepare();
    let assets = FilesystemAssets::new(Some(&base));
    let error = assets
        .resolve(request("font", "../font.ttf", AssetKind::Font), FONT.len() - 1)
        .expect_err("bounded filesystem read");
    assert_eq!(error, ResolveError::TooLarge);
}

#[test]
fn filesystem_adapter_rejects_traversal_beyond_discovered_project_root() {
    let outer = TestProject::new();
    std::fs::write(outer.root().join("outside.ttf"), FONT).expect("outside file");
    let root = outer.root().join("project");
    std::fs::create_dir(&root).expect("project");
    std::fs::write(root.join("Cargo.toml"), "").expect("marker");
    let assets = FilesystemAssets::new(Some(&root));
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    assert_eq!(
        session
            .resolve(request("font", "../outside.ttf", AssetKind::Font))
            .expect_err("outside project")
            .code(),
        "asset-outside-project"
    );
}

#[cfg(unix)]
#[test]
fn filesystem_adapter_rejects_symlink_escape_and_accepts_contained_symlink() {
    let outside = TestProject::new();
    std::fs::write(outside.root().join("font.ttf"), FONT).expect("outside font");
    let project = TestProject::new();
    let base = project.prepare();
    std::os::unix::fs::symlink(outside.root().join("font.ttf"), base.join("escape.ttf"))
        .expect("escaping symlink");
    std::os::unix::fs::symlink(project.root().join("font.ttf"), base.join("inside.ttf"))
        .expect("contained symlink");
    let assets = FilesystemAssets::new(Some(&base));
    let mut session = AssetSession::new(&assets, AssetLimits::default());
    assert_eq!(
        session
            .resolve(request("font", "escape.ttf", AssetKind::Font))
            .expect_err("symlink escape")
            .code(),
        "asset-outside-project"
    );
    assert_eq!(
        session
            .resolve(request("font", "inside.ttf", AssetKind::Font))
            .expect("contained symlink"),
        FONT
    );
}
