use std::borrow::Cow;
use std::path::Path;

use rive_cli::builder::SceneSpec;
use rive_cli::compile::assets::{
    AssetKind, AssetLimits, AssetRequest, AssetResolver, MemoryAssets, ResolveError,
};
use rive_cli::compile::{
    CompileOptions, EmbeddedCompileError, compile_scene, compile_scene_with_assets,
};
use rive_cli::objects::core::type_keys;
use rive_cli::validator::{InspectFilter, PropertyValueRead, parse_riv, validate_riv};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const FONT: &[u8] = include_bytes!("../assets/fonts/Inter-Bold-Subset.ttf");
const IMAGE: &[u8] = include_bytes!("../assets/textures/aurora.png");
const FONT_KEY: &str = "memory://font";
const IMAGE_KEY: &str = "memory://image";
const FILE_ID: u64 = 0x1020_3040;

fn document() -> Value {
    let mut document: Value =
        serde_json::from_str(include_str!("fixtures/embedded_assets.json")).expect("fixture JSON");
    document["artboard"]["children"][0]["source"] = json!(FONT_KEY);
    document["artboard"]["children"][1]["source"] = json!(IMAGE_KEY);
    document
}

fn spec() -> SceneSpec {
    serde_json::from_value(document()).expect("SceneSpec")
}

fn assets() -> MemoryAssets {
    let mut assets = MemoryAssets::new();
    assets.insert(FONT_KEY, FONT);
    assets.insert(IMAGE_KEY, IMAGE);
    assets
}

fn options() -> CompileOptions {
    CompileOptions {
        file_id: FILE_ID,
        asset_limits: AssetLimits::default(),
    }
}

#[test]
fn public_memory_compilation_embeds_image_and_font_without_a_base_directory() {
    let bytes = compile_scene_with_assets(&spec(), options(), &assets()).expect("memory compile");
    let report = validate_riv(&bytes).expect("standard Rive bytes");
    assert!(report.valid, "{report:?}");
    assert_eq!(report.header.file_id, FILE_ID);
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("parse assets");
    let lengths = parsed
        .objects
        .iter()
        .filter(|object| object.type_key == type_keys::FILE_ASSET_CONTENTS)
        .flat_map(|object| &object.properties)
        .filter_map(|property| match &property.value {
            PropertyValueRead::Bytes { length } => Some(*length),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(lengths, [FONT.len(), IMAGE.len()]);
}

#[test]
fn explicit_options_and_identical_bytes_produce_identical_binaries() {
    let spec = spec();
    let assets = assets();
    let first = compile_scene_with_assets(&spec, options(), &assets).expect("first compile");
    let second = compile_scene_with_assets(&spec, options(), &assets).expect("second compile");
    assert_eq!(first, second);
}

#[test]
fn legacy_path_entry_and_memory_entry_share_asset_order_file_id_and_bytes() {
    let scene: SceneSpec =
        serde_json::from_str(include_str!("fixtures/embedded_assets.json")).expect("legacy scene");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let disk = compile_scene(&scene, Some(&root.join("tests/fixtures")), FILE_ID)
        .expect("filesystem compile");
    let memory = compile_scene_with_assets(&spec(), options(), &assets()).expect("memory compile");
    assert_eq!(memory, disk);
}

#[test]
fn memory_compilation_never_falls_back_to_an_existing_disk_source() {
    let scene: SceneSpec = serde_json::from_str(include_str!("fixtures/embedded_assets.json"))
        .expect("disk-named scene");
    let error = compile_scene_with_assets(&scene, options(), &MemoryAssets::new())
        .expect_err("no ambient asset lookup");
    assert_eq!(error.code(), "asset-missing");
    match error {
        EmbeddedCompileError::Asset(error) => {
            assert_eq!(error.asset_name, "InterBold");
            assert_eq!(error.source_key, "../../assets/fonts/Inter-Bold-Subset.ttf");
        }
        other => panic!("expected logical asset diagnostic, got {other:?}"),
    }
}

#[test]
fn empty_wrong_kind_and_budget_failures_retain_logical_asset_identity() {
    for (bytes, code) in [(&[][..], "asset-empty"), (IMAGE, "asset-kind-mismatch")] {
        let mut assets = assets();
        assets.insert(FONT_KEY, bytes);
        let error =
            compile_scene_with_assets(&spec(), options(), &assets).expect_err("invalid font");
        assert_eq!(error.code(), code);
        assert!(
            matches!(error, EmbeddedCompileError::Asset(ref error) if error.asset_name == "InterBold")
        );
    }
    let error = compile_scene_with_assets(
        &spec(),
        CompileOptions {
            file_id: FILE_ID,
            asset_limits: AssetLimits {
                per_asset_bytes: FONT.len() - 1,
                total_bytes: usize::MAX,
            },
        },
        &assets(),
    )
    .expect_err("per-asset budget");
    assert_eq!(error.code(), "asset-too-large");
    let error = compile_scene_with_assets(
        &spec(),
        CompileOptions {
            file_id: FILE_ID,
            asset_limits: AssetLimits {
                per_asset_bytes: usize::MAX,
                total_bytes: FONT.len() + IMAGE.len() - 1,
            },
        },
        &assets(),
    )
    .expect_err("total budget");
    assert_eq!(error.code(), "asset-total-budget-exceeded");
    assert!(
        matches!(error, EmbeddedCompileError::Asset(ref error) if error.asset_name == "Aurora")
    );
}

struct NoReads;

impl AssetResolver for NoReads {
    fn resolve<'a>(
        &'a self,
        _request: AssetRequest<'_>,
        _max_bytes: usize,
    ) -> Result<Cow<'a, [u8]>, ResolveError> {
        panic!("invalid scene must fail before asset resolution")
    }
}

#[test]
fn structural_validation_precedes_caller_asset_resolution() {
    let mut document = document();
    document["artboard"]["children"][1]["name"] = json!("InterBold");
    let invalid = serde_json::from_value(document).expect("invalid semantic scene");
    let error =
        compile_scene_with_assets(&invalid, options(), &NoReads).expect_err("duplicate name");
    assert_eq!(error.code(), "invalid-scene");
    assert!(matches!(error, EmbeddedCompileError::Build(_)));
}

#[test]
fn an_explicit_external_asset_declaration_does_not_request_missing_embedded_bytes() {
    let mut document = document();
    for index in [0, 1] {
        document["artboard"]["children"][index]
            .as_object_mut()
            .expect("asset")
            .remove("source");
    }
    let scene = serde_json::from_value(document).expect("external asset scene");
    let bytes =
        compile_scene_with_assets(&scene, options(), &NoReads).expect("no embedded sources");
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("external asset binary");
    assert!(
        !parsed
            .objects
            .iter()
            .any(|object| object.type_key == type_keys::FILE_ASSET_CONTENTS)
    );
}

#[test]
fn public_asset_requests_preserve_kind_and_source() {
    struct CheckedAssets;
    impl AssetResolver for CheckedAssets {
        fn resolve<'a>(
            &'a self,
            request: AssetRequest<'_>,
            _max_bytes: usize,
        ) -> Result<Cow<'a, [u8]>, ResolveError> {
            match (request.name, request.source, request.kind) {
                ("InterBold", FONT_KEY, AssetKind::Font) => Ok(Cow::Borrowed(FONT)),
                ("Aurora", IMAGE_KEY, AssetKind::Image) => Ok(Cow::Borrowed(IMAGE)),
                _ => panic!("unexpected request {request:?}"),
            }
        }
    }
    compile_scene_with_assets(&spec(), options(), &CheckedAssets).expect("typed requests");
}

#[test]
fn successful_legacy_scene_and_authoring_fixture_digests_remain_unchanged() {
    let baseline: Value =
        serde_json::from_str(include_str!("evidence/embedded-assets/legacy-digests.json"))
            .expect("baseline captured on the unchanged compiler");
    let file_id = baseline["file_id"].as_u64().expect("baseline file ID");
    let digests = baseline["digests"].as_object().expect("baseline digests");
    assert!(!digests.is_empty());
    assert!(digests.contains_key("tests/fixtures/embedded_assets.json"));
    assert!(
        digests
            .keys()
            .any(|path| path.starts_with("examples/authoring/"))
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (relative, expected) in digests {
        let path = root.join(relative);
        let input = std::fs::read_to_string(&path).expect("baseline fixture");
        let document: Value = serde_json::from_str(&input).expect("fixture JSON");
        let scene: SceneSpec = if document.get("authoring_format_version").is_some() {
            let lowered =
                rive_cli::authoring::lower_authoring_json(&input).expect("authoring lowering");
            serde_json::from_value(lowered.scene).expect("lowered scene")
        } else {
            serde_json::from_value(document).expect("scene")
        };
        let bytes =
            compile_scene(&scene, path.parent(), file_id).expect("legacy successful fixture");
        assert_eq!(
            format!("{:x}", Sha256::digest(bytes)),
            expected.as_str().expect("digest"),
            "{relative}"
        );
    }
}

#[test]
fn total_asset_budget_is_file_scoped_across_artboards() {
    let scene: SceneSpec = serde_json::from_value(json!({
        "scene_format_version": 1,
        "artboards": [
            {"name": "First", "width": 64, "height": 64, "children": [
                {"type": "font_asset", "name": "FirstFont", "source": FONT_KEY}
            ]},
            {"name": "Second", "width": 64, "height": 64, "children": [
                {"type": "font_asset", "name": "SecondFont", "source": FONT_KEY}
            ]}
        ]
    }))
    .expect("two artboards");
    let error = compile_scene_with_assets(
        &scene,
        CompileOptions {
            file_id: FILE_ID,
            asset_limits: AssetLimits {
                per_asset_bytes: FONT.len(),
                total_bytes: FONT.len() * 2 - 1,
            },
        },
        &assets(),
    )
    .expect_err("two emitted payloads exceed the file budget");
    assert_eq!(error.code(), "asset-total-budget-exceeded");
    assert!(
        matches!(error, EmbeddedCompileError::Asset(ref error) if error.asset_name == "SecondFont")
    );
}

#[test]
fn duplicate_file_scoped_asset_names_fail_before_any_resolver_call() {
    let scene: SceneSpec = serde_json::from_value(json!({
        "scene_format_version": 1,
        "artboards": [
            {"name": "First", "width": 64, "height": 64, "children": [
                {"type": "font_asset", "name": "SharedFont", "source": FONT_KEY}
            ]},
            {"name": "Second", "width": 64, "height": 64, "children": [
                {"type": "font_asset", "name": "SharedFont", "source": FONT_KEY}
            ]}
        ]
    }))
    .expect("two artboards with duplicate file assets");
    let error =
        compile_scene_with_assets(&scene, options(), &NoReads).expect_err("file-scope duplicate");
    assert_eq!(error.code(), "invalid-scene");
}

#[test]
fn nested_assets_are_rejected_before_any_resolver_callback() {
    for kind in ["image_asset", "font_asset", "audio_asset"] {
        for container in ["shape", "node", "solo"] {
            for separate_artboard in [false, true] {
                let invalid_child = json!({
                    "type": container,
                    "name": "Container",
                    "children": [{"type": kind, "name": "NestedAsset"}]
                });
                let mut artboards = vec![json!({
                    "name": "First", "width": 64, "height": 64,
                    "children": [{"type": "font_asset", "name": "FirstFont", "source": FONT_KEY}]
                })];
                if separate_artboard {
                    artboards.push(json!({
                        "name": "Second", "width": 64, "height": 64,
                        "children": [invalid_child]
                    }));
                } else {
                    artboards[0]["children"]
                        .as_array_mut()
                        .expect("children")
                        .push(invalid_child);
                }
                let scene = serde_json::from_value(json!({
                    "scene_format_version": 1,
                    "artboards": artboards
                }))
                .expect("syntactically valid nested asset scene");
                let error = compile_scene_with_assets(&scene, options(), &NoReads)
                    .expect_err("invalid placement must fail before caller code runs");
                assert_eq!(error.code(), "invalid-scene");
                assert!(error.to_string().contains("NestedAsset"), "{error}");
            }
        }
    }
}
