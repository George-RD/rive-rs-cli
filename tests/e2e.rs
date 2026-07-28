use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixture_path(name: &str) -> PathBuf {
    project_root().join("tests").join("fixtures").join(name)
}

fn temp_output(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rive_cli_{}.riv", name))
}

fn cleanup(path: &Path) {
    let _ = fs::remove_file(path);
}

fn cargo_run(args: &[&str]) -> Output {
    static BINARY: OnceLock<PathBuf> = OnceLock::new();
    let binary = BINARY.get_or_init(|| {
        let status = Command::new("cargo")
            .args(["build", "--quiet", "--bin", "rive-cli"])
            .current_dir(project_root())
            .status()
            .expect("failed to build rive-cli");
        assert!(status.success(), "failed to build rive-cli");
        project_root()
            .join("target")
            .join("debug")
            .join(if cfg!(windows) {
                "rive-cli.exe"
            } else {
                "rive-cli"
            })
    });
    Command::new(binary)
        .args(args)
        .current_dir(project_root())
        .output()
        .expect("failed to run rive-cli")
}

fn read_json(path: &Path) -> serde_json::Value {
    serde_json::from_str(&fs::read_to_string(path).expect("failed to read JSON"))
        .expect("failed to parse JSON")
}

fn assert_success(result: &Output) {
    assert!(
        result.status.success(),
        "command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

fn assert_failure(result: &Output) {
    assert!(
        !result.status.success(),
        "command unexpectedly succeeded\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

fn generated_output(name: &str) -> PathBuf {
    let output = temp_output(name);
    cleanup(&output);
    output
}

fn generate_fixture(fixture: &str, output_name: &str) -> PathBuf {
    let output = generated_output(output_name);
    let result = cargo_run(&[
        "generate",
        fixture_path(fixture).to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    output
}

fn inspect_json(path: &Path) -> serde_json::Value {
    let result = cargo_run(&["inspect", path.to_str().unwrap(), "--json"]);
    assert_success(&result);
    serde_json::from_slice(&result.stdout).expect("invalid inspect JSON")
}

fn decompile_json(path: &Path) -> serde_json::Value {
    let result = cargo_run(&["decompile", path.to_str().unwrap()]);
    assert_success(&result);
    serde_json::from_slice(&result.stdout).expect("invalid decompile JSON")
}

fn find_objects<'a>(value: &'a serde_json::Value, type_name: &str) -> Vec<&'a serde_json::Value> {
    value["objects"]
        .as_array()
        .expect("missing objects")
        .iter()
        .filter(|object| object["type_name"] == type_name)
        .collect()
}

fn property_u64(object: &serde_json::Value, key: &str) -> Option<u64> {
    object["properties"]
        .as_array()?
        .iter()
        .find(|property| property["name"] == key)
        .and_then(|property| property["value"].as_u64())
}

fn property_f64(object: &serde_json::Value, key: &str) -> Option<f64> {
    object["properties"]
        .as_array()?
        .iter()
        .find(|property| property["name"] == key)
        .and_then(|property| property["value"].as_f64())
}

fn property_string<'a>(object: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    object["properties"]
        .as_array()?
        .iter()
        .find(|property| property["name"] == key)
        .and_then(|property| property["value"].as_str())
}

fn object_type_names(value: &serde_json::Value) -> Vec<&str> {
    value["objects"]
        .as_array()
        .expect("missing objects")
        .iter()
        .filter_map(|object| object["type_name"].as_str())
        .collect()
}

fn validate(path: &Path) -> Output {
    cargo_run(&["validate", path.to_str().unwrap()])
}

fn generate_and_validate(fixture: &str, name: &str) -> PathBuf {
    let output = generate_fixture(fixture, name);
    assert_success(&validate(&output));
    output
}

#[test]
fn test_help_output() {
    let result = cargo_run(&["--help"]);
    assert_success(&result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Generate Rive .riv files programmatically"));
    assert!(stdout.contains("generate"));
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("inspect"));
}

#[test]
fn test_version_flag() {
    let result = cargo_run(&["--version"]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_generate_help() {
    let result = cargo_run(&["generate", "--help"]);
    assert_success(&result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Generate a .riv file"));
    assert!(stdout.contains("--file-id"));
}

#[test]
fn test_generate_no_args() {
    let result = cargo_run(&["generate"]);
    assert_failure(&result);
}

#[test]
fn test_generate_missing_input_file() {
    let output = generated_output("missing_input");
    let result = cargo_run(&[
        "generate",
        "tests/fixtures/does-not-exist.json",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("error reading"));
}

#[test]
fn test_generate_malformed_json() {
    let root = std::env::temp_dir().join("rive_cli_malformed");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let input = root.join("scene.json");
    fs::write(&input, "{ not-json }").unwrap();
    let output = root.join("out.riv");
    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("error parsing JSON"));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_generate_invalid_scene_spec() {
    let root = std::env::temp_dir().join("rive_cli_invalid_scene");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let input = root.join("scene.json");
    fs::write(
        &input,
        r#"{
          "scene_format_version": 1,
          "artboard": {"name":"Main","width":0,"height":100,"children":[]}
        }"#,
    )
    .unwrap();
    let output = root.join("out.riv");
    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("invalid scene spec"));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_generate_json_output() {
    let output = generated_output("json_output");
    let result = cargo_run(&[
        "generate",
        fixture_path("minimal.json").to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        "--json",
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(value["ok"], true);
    assert_eq!(value["output_path"], output.display().to_string());
    assert!(value["bytes_written"].as_u64().unwrap() > 0);
    cleanup(&output);
}

#[test]
fn test_generate_minimal() {
    let output = generate_and_validate("minimal.json", "minimal");
    cleanup(&output);
}

#[test]
fn test_generate_empty_artboard() {
    let output = generate_and_validate("empty_artboard.json", "empty_artboard");
    cleanup(&output);
}

#[test]
fn test_generate_shapes() {
    let output = generate_and_validate("shapes.json", "shapes");
    cleanup(&output);
}

#[test]
fn test_generate_gradients() {
    let output = generate_and_validate("gradients.json", "gradients");
    cleanup(&output);
}

#[test]
fn test_generate_animation() {
    let output = generate_and_validate("animation.json", "animation");
    cleanup(&output);
}

#[test]
fn test_generate_state_machine() {
    let output = generate_and_validate("state_machine.json", "state_machine");
    cleanup(&output);
}

#[test]
fn test_generate_text() {
    let output = generate_and_validate("text.json", "text");
    cleanup(&output);
}

#[test]
fn test_generate_assets() {
    let output = generate_and_validate("assets.json", "assets");
    cleanup(&output);
}

#[test]
fn test_generate_layout() {
    let output = generate_and_validate("layout.json", "layout");
    cleanup(&output);
}

#[test]
fn test_generate_data_binding() {
    let output = generate_and_validate("data_binding.json", "data_binding");
    cleanup(&output);
}

#[test]
fn test_generate_bones() {
    let output = generate_and_validate("bones.json", "bones");
    cleanup(&output);
}

#[test]
fn test_generate_constraints() {
    let output = generate_and_validate("constraints.json", "constraints");
    cleanup(&output);
}

#[test]
fn test_generate_multi_artboard() {
    let output = generate_and_validate("multi_artboard.json", "multi_artboard");
    cleanup(&output);
}

#[test]
fn test_generate_nested_artboard() {
    let output = generate_and_validate("nested_artboard.json", "nested_artboard");
    cleanup(&output);
}

#[test]
fn test_generate_path() {
    let output = generate_and_validate("path.json", "path");
    cleanup(&output);
}

#[test]
fn test_generate_trim_path() {
    let output = generate_and_validate("trim_path.json", "trim_path");
    cleanup(&output);
}

#[test]
fn test_generate_color_animation() {
    let output = generate_and_validate("color_animation.json", "color_animation");
    cleanup(&output);
}

#[test]
fn test_generate_stroke_styles() {
    let output = generate_and_validate("stroke_styles.json", "stroke_styles");
    cleanup(&output);
}

#[test]
fn test_generate_button_states() {
    let output = generate_and_validate("button_states.json", "button_states");
    cleanup(&output);
}

#[test]
fn test_generate_game_hud() {
    let output = generate_and_validate("game_hud.json", "game_hud");
    cleanup(&output);
}

#[test]
fn test_generate_icon_set() {
    let output = generate_and_validate("icon_set.json", "icon_set");
    cleanup(&output);
}

#[test]
fn test_generate_loader() {
    let output = generate_and_validate("loader.json", "loader");
    cleanup(&output);
}

#[test]
fn test_generate_mascot() {
    let output = generate_and_validate("mascot.json", "mascot");
    cleanup(&output);
}

#[test]
fn test_generate_loop_animation() {
    let output = generate_and_validate("loop_animation.json", "loop_animation");
    cleanup(&output);
}

#[test]
fn test_generate_image_node() {
    let output = generate_and_validate("image_node.json", "image_node");
    cleanup(&output);
}

#[test]
fn test_generate_mesh() {
    let output = generate_and_validate("mesh.json", "mesh");
    cleanup(&output);
}

#[test]
fn test_generate_nslicer() {
    let output = generate_and_validate("nslicer.json", "nslicer");
    cleanup(&output);
}

#[test]
fn test_generate_scripting() {
    let output = generate_and_validate("scripting.json", "scripting");
    cleanup(&output);
}

#[test]
fn test_generate_effects() {
    let output = generate_and_validate("effects.json", "effects");
    cleanup(&output);
}

#[test]
fn test_generate_graphics_misc() {
    let output = generate_and_validate("graphics_misc.json", "graphics_misc");
    cleanup(&output);
}

#[test]
fn test_generate_events_extended() {
    let output = generate_and_validate("events_extended.json", "events_extended");
    cleanup(&output);
}

#[test]
fn test_generate_layout_extensions() {
    let output = generate_and_validate("layout_extensions.json", "layout_extensions");
    cleanup(&output);
}

#[test]
fn test_generate_nested_extensions() {
    let output = generate_and_validate("nested_extensions.json", "nested_extensions");
    cleanup(&output);
}

#[test]
fn test_generate_asset_extensions() {
    let output = generate_and_validate("asset_extensions.json", "asset_extensions");
    cleanup(&output);
}

#[test]
fn test_generate_data_converters() {
    let output = generate_and_validate("data_converters.json", "data_converters");
    cleanup(&output);
}

#[test]
fn test_generate_new_constraints() {
    let output = generate_and_validate("new_constraints.json", "new_constraints");
    cleanup(&output);
}

#[test]
fn test_generate_invalid_input() {
    let result = cargo_run(&[
        "generate",
        fixture_path("invalid_input.json").to_str().unwrap(),
    ]);
    assert_failure(&result);
}

#[test]
fn test_generate_invalid_missing_version() {
    let result = cargo_run(&[
        "generate",
        fixture_path("invalid_missing_version.json").to_str().unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("scene_format_version"));
}

#[test]
fn test_generate_invalid_missing_names() {
    let result = cargo_run(&[
        "generate",
        fixture_path("invalid_missing_names.json").to_str().unwrap(),
    ]);
    assert_failure(&result);
}

#[test]
fn test_generate_invalid_bad_values() {
    let result = cargo_run(&[
        "generate",
        fixture_path("invalid_bad_values.json").to_str().unwrap(),
    ]);
    assert_failure(&result);
}

#[test]
fn test_generate_artboard_preset() {
    let output = generate_and_validate("artboard_preset.json", "artboard_preset");
    cleanup(&output);
}

#[test]
fn test_generate_accepts_ellipse_parametric_width_keyframe() {
    let output = generate_and_validate(
        "ellipse_parametric_width_animation.json",
        "ellipse_parametric_width",
    );
    cleanup(&output);
}

#[test]
fn test_generate_rejects_shape_parametric_width_keyframe() {
    let result = cargo_run(&[
        "generate",
        fixture_path("shape_invalid_width_animation.json")
            .to_str()
            .unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("not animatable for shape"));
}

#[test]
fn test_generate_rejects_state_machine_layer_without_exit() {
    let result = cargo_run(&[
        "generate",
        fixture_path("state_machine_missing_exit.json")
            .to_str()
            .unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("requires exactly one exit state"));
}

#[test]
fn test_generate_validate_comparison_trim() {
    let output = generate_and_validate("comparison_trim.json", "comparison_trim");
    cleanup(&output);
}

#[test]
fn test_generate_validate_comparison_clip_tests() {
    let output = generate_and_validate("comparison_clip_tests.json", "comparison_clip_tests");
    cleanup(&output);
}

#[test]
fn test_generate_validate_comparison_quantize_test() {
    let output = generate_and_validate("comparison_quantize_test.json", "comparison_quantize_test");
    cleanup(&output);
}

#[test]
fn test_generate_validate_comparison_official_test() {
    let output = generate_and_validate("comparison_official_test.json", "comparison_official");
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_blend_animation() {
    let output = generate_and_validate("blend_animation.json", "blend_animation");
    let inspected = inspect_json(&output);
    let names = object_type_names(&inspected);
    assert!(names.contains(&"BlendState1D"));
    assert!(names.contains(&"BlendAnimation1D"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_clipping_shape() {
    let output = generate_and_validate("clipping_shape.json", "clipping_shape");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"ClippingShape"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_cubic_asymmetric() {
    let output = generate_and_validate("cubic_asymmetric.json", "cubic_asymmetric");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"CubicAsymmetricVertex"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_draw_rules() {
    let output = generate_and_validate("draw_rules.json", "draw_rules");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"DrawTarget"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_elastic_interpolator() {
    let output = generate_and_validate("elastic_interpolator.json", "elastic_interpolator");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"ElasticInterpolator"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_event_test() {
    let output = generate_and_validate("event_test.json", "event_test");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Event"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_follow_path_constraint() {
    let output = generate_and_validate("follow_path_constraint.json", "follow_path_constraint");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"FollowPathConstraint"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_game_hud() {
    let output = generate_and_validate("game_hud.json", "game_hud_inspect");
    let inspected = inspect_json(&output);
    assert!(!find_objects(&inspected, "Text").is_empty());
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_icon_set() {
    let output = generate_and_validate("icon_set.json", "icon_set_inspect");
    let inspected = inspect_json(&output);
    assert!(find_objects(&inspected, "Artboard").len() >= 2);
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_joystick() {
    let output = generate_and_validate("joystick.json", "joystick");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Joystick"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_keyframe_types() {
    let output = generate_and_validate("keyframe_types.json", "keyframe_types");
    let inspected = inspect_json(&output);
    let names = object_type_names(&inspected);
    assert!(names.contains(&"KeyFrameBool"));
    assert!(names.contains(&"KeyFrameColor"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_mascot() {
    let output = generate_and_validate("mascot.json", "mascot_inspect");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Bone"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_nested_simple_animation() {
    let output = generate_and_validate("nested_simple_animation.json", "nested_simple_animation");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"NestedSimpleAnimation"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_polygon_star() {
    let output = generate_and_validate("polygon_star.json", "polygon_star");
    let inspected = inspect_json(&output);
    let names = object_type_names(&inspected);
    assert!(names.contains(&"Polygon"));
    assert!(names.contains(&"Star"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_text_modifiers() {
    let output = generate_and_validate("text_modifiers.json", "text_modifiers");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"TextModifierGroup"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_transition_comparators() {
    let output = generate_and_validate("transition_comparators.json", "transition_comparators");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"TransitionNumberCondition"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_triangle() {
    let output = generate_and_validate("triangle.json", "triangle");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Triangle"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_view_model_instances() {
    let output = generate_and_validate("view_model_instances.json", "view_model_instances");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"ViewModelInstance"));
    cleanup(&output);
}

#[test]
fn test_generate_validate_listener_test() {
    let output = generate_and_validate("listener_test.json", "listener_test");
    cleanup(&output);
}

#[test]
fn test_generate_validate_points_path() {
    let output = generate_and_validate("points_path.json", "points_path");
    cleanup(&output);
}

#[test]
fn test_generate_validate_solo_test() {
    let output = generate_and_validate("solo_test.json", "solo_test");
    cleanup(&output);
}

#[test]
fn test_generate_then_validate_then_inspect() {
    let output = generate_fixture("minimal.json", "generate_validate_inspect");
    assert_success(&validate(&output));
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Artboard"));
    cleanup(&output);
}

#[test]
fn test_generate_then_decompile_roundtrip() {
    let output = generate_fixture("shapes.json", "generate_decompile_roundtrip");
    let decompiled = decompile_json(&output);
    assert!(object_type_names(&decompiled).contains(&"Shape"));
    cleanup(&output);
}

#[test]
fn test_validate_generated_file() {
    let output = generate_fixture("minimal.json", "validate_generated");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_json_output() {
    let output = generate_fixture("minimal.json", "validate_json");
    let result = cargo_run(&["validate", output.to_str().unwrap(), "--json"]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(value["ok"], true);
    assert_eq!(value["valid"], true);
    cleanup(&output);
}

#[test]
fn test_validate_missing_file() {
    let result = cargo_run(&["validate", "does-not-exist.riv"]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("error reading"));
}

#[test]
fn test_validate_invalid_file() {
    let path = std::env::temp_dir().join("rive_cli_invalid.riv");
    fs::write(&path, b"not a rive file").unwrap();
    let result = validate(&path);
    assert_failure(&result);
    cleanup(&path);
}

#[test]
fn test_validate_truncated_file() {
    let path = std::env::temp_dir().join("rive_cli_truncated.riv");
    fs::write(&path, b"RIVE").unwrap();
    let result = validate(&path);
    assert_failure(&result);
    cleanup(&path);
}

#[test]
fn test_validate_warns_on_version_mismatch() {
    let output = generate_fixture("minimal.json", "version_warning");
    let mut bytes = fs::read(&output).unwrap();
    bytes[4] = 6;
    fs::write(&output, bytes).unwrap();
    let result = validate(&output);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("warning"));
    cleanup(&output);
}

#[test]
fn test_validate_empty_artboard() {
    let output = generate_fixture("empty_artboard.json", "validate_empty_artboard");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_artboard_preset() {
    let output = generate_fixture("artboard_preset.json", "validate_artboard_preset");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_assets() {
    let output = generate_fixture("assets.json", "validate_assets");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_bones() {
    let output = generate_fixture("bones.json", "validate_bones");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_button_states() {
    let output = generate_fixture("button_states.json", "validate_button_states");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_color_animation() {
    let output = generate_fixture("color_animation.json", "validate_color_animation");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_constraints() {
    let output = generate_fixture("constraints.json", "validate_constraints");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_data_binding() {
    let output = generate_fixture("data_binding.json", "validate_data_binding");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_data_converters() {
    let output = generate_fixture("data_converters.json", "validate_data_converters");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_game_hud() {
    let output = generate_fixture("game_hud.json", "validate_game_hud");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_gradients() {
    let output = generate_fixture("gradients.json", "validate_gradients");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_icon_set() {
    let output = generate_fixture("icon_set.json", "validate_icon_set");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_image_node() {
    let output = generate_fixture("image_node.json", "validate_image_node");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_layout() {
    let output = generate_fixture("layout.json", "validate_layout");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_loader() {
    let output = generate_fixture("loader.json", "validate_loader");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_loop_animation() {
    let output = generate_fixture("loop_animation.json", "validate_loop_animation");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_mascot() {
    let output = generate_fixture("mascot.json", "validate_mascot");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_multi_artboard() {
    let output = generate_fixture("multi_artboard.json", "validate_multi_artboard");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_stroke_styles() {
    let output = generate_fixture("stroke_styles.json", "validate_stroke_styles");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_text() {
    let output = generate_fixture("text.json", "validate_text");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_validate_scripting() {
    let output = generate_fixture("scripting.json", "validate_scripting");
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_decompile_missing_file() {
    let result = cargo_run(&["decompile", "does-not-exist.riv"]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("error reading"));
}

#[test]
fn test_decompile_corrupt_file() {
    let path = std::env::temp_dir().join("rive_cli_corrupt.riv");
    fs::write(&path, b"broken").unwrap();
    let result = cargo_run(&["decompile", path.to_str().unwrap()]);
    assert_failure(&result);
    cleanup(&path);
}

#[test]
fn test_decompile_outputs_resolved_names_json() {
    let output = generate_fixture("shapes.json", "decompile_names");
    let value = decompile_json(&output);
    let objects = value["objects"].as_array().unwrap();
    assert!(objects.iter().any(|object| object["type_name"] == "Shape"));
    cleanup(&output);
}

#[test]
fn test_decompile_roundtrip_verify_objects() {
    let output = generate_fixture("data_binding.json", "decompile_objects");
    let value = decompile_json(&output);
    let names = object_type_names(&value);
    assert!(names.contains(&"ViewModel"));
    assert!(names.contains(&"DataBind"));
    cleanup(&output);
}

#[test]
fn test_inspect_missing_file() {
    let result = cargo_run(&["inspect", "does-not-exist.riv"]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("error reading"));
}

#[test]
fn test_inspect_generated_file() {
    let output = generate_fixture("minimal.json", "inspect_generated");
    let result = cargo_run(&["inspect", output.to_str().unwrap()]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("Artboard"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_flag() {
    let output = generate_fixture("minimal.json", "inspect_json_flag");
    let result = cargo_run(&["inspect", output.to_str().unwrap(), "--json"]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert!(value["objects"].is_array());
    cleanup(&output);
}

#[test]
fn test_inspect_help() {
    let result = cargo_run(&["inspect", "--help"]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("--type-name"));
}

#[test]
fn test_inspect_empty_artboard() {
    let output = generate_fixture("empty_artboard.json", "inspect_empty_artboard");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Artboard"));
    cleanup(&output);
}

#[test]
fn test_inspect_bones() {
    let output = generate_fixture("bones.json", "inspect_bones");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Bone"));
    cleanup(&output);
}

#[test]
fn test_inspect_button_states() {
    let output = generate_fixture("button_states.json", "inspect_button_states");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"StateMachine"));
    cleanup(&output);
}

#[test]
fn test_inspect_color_animation() {
    let output = generate_fixture("color_animation.json", "inspect_color_animation");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"KeyFrameColor"));
    cleanup(&output);
}

#[test]
fn test_inspect_constraints() {
    let output = generate_fixture("constraints.json", "inspect_constraints");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"IKConstraint"));
    cleanup(&output);
}

#[test]
fn test_inspect_game_hud() {
    let output = generate_fixture("game_hud.json", "inspect_game_hud");
    let inspected = inspect_json(&output);
    assert!(!find_objects(&inspected, "Text").is_empty());
    cleanup(&output);
}

#[test]
fn test_inspect_gradients() {
    let output = generate_fixture("gradients.json", "inspect_gradients");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"LinearGradient"));
    cleanup(&output);
}

#[test]
fn test_inspect_icon_set() {
    let output = generate_fixture("icon_set.json", "inspect_icon_set");
    let inspected = inspect_json(&output);
    assert!(find_objects(&inspected, "Artboard").len() >= 2);
    cleanup(&output);
}

#[test]
fn test_inspect_image_node() {
    let output = generate_fixture("image_node.json", "inspect_image_node");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Image"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_assets() {
    let output = generate_fixture("assets.json", "inspect_json_assets");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"ImageAsset"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_bones() {
    let output = generate_fixture("bones.json", "inspect_json_bones");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Bone"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_constraints() {
    let output = generate_fixture("constraints.json", "inspect_json_constraints");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"IKConstraint"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_data_binding() {
    let output = generate_fixture("data_binding.json", "inspect_json_data_binding");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"DataBind"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_image_node() {
    let output = generate_fixture("image_node.json", "inspect_json_image_node");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Image"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_layout() {
    let output = generate_fixture("layout.json", "inspect_json_layout");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"LayoutComponent"));
    cleanup(&output);
}

#[test]
fn test_inspect_json_multi_artboard() {
    let output = generate_fixture("multi_artboard.json", "inspect_json_multi_artboard");
    let inspected = inspect_json(&output);
    assert!(find_objects(&inspected, "Artboard").len() >= 2);
    cleanup(&output);
}

#[test]
fn test_inspect_json_text() {
    let output = generate_fixture("text.json", "inspect_json_text");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Text"));
    cleanup(&output);
}

#[test]
fn test_inspect_layout() {
    let output = generate_fixture("layout.json", "inspect_layout");
    let result = cargo_run(&["inspect", output.to_str().unwrap()]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("LayoutComponent"));
    cleanup(&output);
}

#[test]
fn test_inspect_loader() {
    let output = generate_fixture("loader.json", "inspect_loader");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"LinearAnimation"));
    cleanup(&output);
}

#[test]
fn test_inspect_loop_animation() {
    let output = generate_fixture("loop_animation.json", "inspect_loop_animation");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"LinearAnimation"));
    cleanup(&output);
}

#[test]
fn test_inspect_mascot() {
    let output = generate_fixture("mascot.json", "inspect_mascot");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Bone"));
    cleanup(&output);
}

#[test]
fn test_inspect_multi_artboard() {
    let output = generate_fixture("multi_artboard.json", "inspect_multi_artboard");
    let result = cargo_run(&["inspect", output.to_str().unwrap()]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("Artboard"));
    cleanup(&output);
}

#[test]
fn test_inspect_nested_artboard() {
    let output = generate_fixture("nested_artboard.json", "inspect_nested_artboard");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"NestedArtboard"));
    cleanup(&output);
}

#[test]
fn test_inspect_stroke_styles() {
    let output = generate_fixture("stroke_styles.json", "inspect_stroke_styles");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Stroke"));
    cleanup(&output);
}

#[test]
fn test_inspect_text() {
    let output = generate_fixture("text.json", "inspect_text");
    let inspected = inspect_json(&output);
    assert!(object_type_names(&inspected).contains(&"Text"));
    cleanup(&output);
}

#[test]
fn test_inspect_nonexistent_type_key_graceful() {
    let output = generate_fixture("minimal.json", "inspect_nonexistent_type");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--type-key",
        "65535",
    ]);
    assert_success(&result);
    cleanup(&output);
}

#[test]
fn test_inspect_filter_artboard_index_human_output() {
    let output = generate_fixture("multi_artboard.json", "filter_artboard_index");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--artboard-index",
        "1",
    ]);
    assert_success(&result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("artboard_index=1"));
    cleanup(&output);
}

#[test]
fn test_inspect_filter_artboard_name_and_local_index_json() {
    let output = generate_fixture("multi_artboard.json", "filter_name_local");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--artboard-name",
        "secondary",
        "--local-index",
        "0",
        "--json",
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let objects = value["objects"].as_array().unwrap();
    assert!(!objects.is_empty());
    assert!(objects.iter().all(|object| object["artboard_name"] == "Secondary"));
    assert!(objects.iter().all(|object| object["local_index"] == 0));
    cleanup(&output);
}

#[test]
fn test_inspect_filter_type_key_json() {
    let output = generate_fixture("shapes.json", "filter_type_key");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--type-key",
        "3",
        "--json",
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let objects = value["objects"].as_array().unwrap();
    assert!(!objects.is_empty());
    assert!(objects.iter().all(|object| object["type_key"] == 3));
    cleanup(&output);
}

#[test]
fn test_inspect_filter_type_name_case_insensitive_json() {
    let output = generate_fixture("shapes.json", "filter_type_name");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--type-name",
        "shape",
        "--json",
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let objects = value["objects"].as_array().unwrap();
    assert!(!objects.is_empty());
    assert!(objects.iter().all(|object| object["type_name"] == "Shape"));
    cleanup(&output);
}

#[test]
fn test_inspect_filter_combined_and_logic_json() {
    let output = generate_fixture("shapes.json", "filter_combined");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--type-name",
        "Shape",
        "--property-key",
        "4",
        "--json",
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let objects = value["objects"].as_array().unwrap();
    assert!(!objects.is_empty());
    for object in objects {
        assert_eq!(object["type_name"], "Shape");
        let properties = object["properties"].as_array().unwrap();
        assert!(properties.iter().all(|property| property["key"] == 4));
    }
    cleanup(&output);
}

#[test]
fn test_inspect_filter_object_index_json() {
    let output = generate_fixture("minimal.json", "filter_object_index");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--object-index",
        "1",
        "--json",
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let objects = value["objects"].as_array().unwrap();
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0]["index"], 1);
    cleanup(&output);
}

#[test]
fn test_inspect_filter_property_key_json() {
    let output = generate_fixture("shapes.json", "filter_property_key");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--property-key",
        "4",
        "--json",
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    for object in value["objects"].as_array().unwrap() {
        for property in object["properties"].as_array().unwrap() {
            assert_eq!(property["key"], 4);
        }
    }
    cleanup(&output);
}

#[test]
fn test_inspect_filter_no_match_human_output() {
    let output = generate_fixture("minimal.json", "filter_no_match");
    let result = cargo_run(&[
        "inspect",
        output.to_str().unwrap(),
        "--type-name",
        "DoesNotExist",
    ]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("0 objects"));
    cleanup(&output);
}

#[test]
fn test_compare_reports_a_perfect_match_against_itself() {
    let output = generate_fixture("loader.json", "compare_self");
    let result = cargo_run(&[
        "compare",
        output.to_str().unwrap(),
        output.to_str().unwrap(),
        "--frames",
        "0,15",
        "--max-pixel-diff",
        "0",
    ]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("max pixel diff: 0.0000%"));
    cleanup(&output);
}

#[test]
fn test_compare_reports_differences_and_still_succeeds_without_a_threshold() {
    let reference = generate_fixture("minimal.json", "compare_reference");
    let candidate = generate_fixture("shapes.json", "compare_candidate");
    let result = cargo_run(&[
        "compare",
        reference.to_str().unwrap(),
        candidate.to_str().unwrap(),
        "--frames",
        "0",
    ]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("type delta"));
    cleanup(&reference);
    cleanup(&candidate);
}

#[test]
fn test_compare_exit_code_is_gated_on_max_pixel_diff() {
    let reference = generate_fixture("minimal.json", "compare_threshold_ref");
    let candidate = generate_fixture("shapes.json", "compare_threshold_candidate");
    let result = cargo_run(&[
        "compare",
        reference.to_str().unwrap(),
        candidate.to_str().unwrap(),
        "--frames",
        "0",
        "--max-pixel-diff",
        "0",
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("pixel diff"));
    cleanup(&reference);
    cleanup(&candidate);
}

#[test]
fn test_compare_rejects_invalid_pixel_threshold() {
    let reference = generate_fixture("minimal.json", "compare_invalid_ref");
    let candidate = generate_fixture("minimal.json", "compare_invalid_candidate");
    let result = cargo_run(&[
        "compare",
        reference.to_str().unwrap(),
        candidate.to_str().unwrap(),
        "--max-pixel-diff",
        "101",
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("between 0 and 100"));
    cleanup(&reference);
    cleanup(&candidate);
}

#[test]
fn test_compare_json_threshold_failure_is_structured() {
    let reference = generate_fixture("minimal.json", "compare_json_ref");
    let candidate = generate_fixture("shapes.json", "compare_json_candidate");
    let result = cargo_run(&[
        "compare",
        reference.to_str().unwrap(),
        candidate.to_str().unwrap(),
        "--frames",
        "0",
        "--max-pixel-diff",
        "0",
        "--json",
    ]);
    assert_failure(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
    assert_eq!(value["ok"], false);
    assert_eq!(value["code"], "pixel-diff-threshold");
    cleanup(&reference);
    cleanup(&candidate);
}

#[test]
fn test_pointer_requires_a_state_machine() {
    let output = generate_fixture("state_machine.json", "pointer_requires_sm");
    let result = cargo_run(&[
        "render",
        output.to_str().unwrap(),
        "--pointer",
        "down:10,10@0",
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("requires --state-machine"));
    cleanup(&output);
}

#[test]
fn test_scheduled_input_failures_fail_the_render() {
    let output = generate_fixture("state_machine.json", "scheduled_input_failure");
    let render_dir = std::env::temp_dir().join("rive_cli_bad_input_render");
    let _ = fs::remove_dir_all(&render_dir);
    let result = cargo_run(&[
        "render",
        output.to_str().unwrap(),
        "--state-machine",
        "State Machine 1",
        "--input",
        "missing=true@0",
        "-o",
        render_dir.to_str().unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("input"));
    cleanup(&output);
    let _ = fs::remove_dir_all(render_dir);
}

#[test]
fn test_pointer_and_scheduled_input_change_the_render() {
    let output = generate_fixture("state_machine.json", "pointer_input_render");
    let base_dir = std::env::temp_dir().join("rive_cli_pointer_base");
    let changed_dir = std::env::temp_dir().join("rive_cli_pointer_changed");
    let _ = fs::remove_dir_all(&base_dir);
    let _ = fs::remove_dir_all(&changed_dir);

    let base = cargo_run(&[
        "render",
        output.to_str().unwrap(),
        "--state-machine",
        "State Machine 1",
        "--frames",
        "0,1",
        "-o",
        base_dir.to_str().unwrap(),
    ]);
    assert_success(&base);

    let changed = cargo_run(&[
        "render",
        output.to_str().unwrap(),
        "--state-machine",
        "State Machine 1",
        "--frames",
        "0,1",
        "--input",
        "toggle=true@1",
        "--pointer",
        "down:10,10@1",
        "-o",
        changed_dir.to_str().unwrap(),
    ]);
    assert_success(&changed);

    assert_eq!(
        fs::read(base_dir.join("frame_0000.png")).unwrap(),
        fs::read(changed_dir.join("frame_0000.png")).unwrap()
    );
    assert_ne!(
        fs::read(base_dir.join("frame_0001.png")).unwrap(),
        fs::read(changed_dir.join("frame_0001.png")).unwrap()
    );

    cleanup(&output);
    let _ = fs::remove_dir_all(base_dir);
    let _ = fs::remove_dir_all(changed_dir);
}

#[test]
fn test_ai_help() {
    let result = cargo_run(&["ai", "--help"]);
    assert_success(&result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("generate"));
    assert!(stdout.contains("lab"));
}

#[test]
fn test_ai_generate_help() {
    let result = cargo_run(&["ai", "generate", "--help"]);
    assert_success(&result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("--prompt"));
    assert!(stdout.contains("--template"));
}

#[test]
fn test_ai_lab_json_flag_in_help() {
    let result = cargo_run(&["ai", "lab", "--help"]);
    assert_success(&result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--provider"));
    assert!(stdout.contains("--model"));
}

#[test]
fn test_ai_generate_template_bounce() {
    let output = generated_output("ai_bounce");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_ai_generate_template_spinner() {
    let output = generated_output("ai_spinner");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "spinner",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_ai_generate_template_pulse() {
    let output = generated_output("ai_pulse");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "pulse",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_ai_generate_template_fade() {
    let output = generated_output("ai_fade");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "fade",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    assert_success(&validate(&output));
    cleanup(&output);
}

#[test]
fn test_ai_generate_dry_run() {
    let result = cargo_run(&["ai", "generate", "--template", "bounce", "--dry-run"]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(value["scene_format_version"], 1);
}

#[test]
fn test_ai_generate_json_output() {
    let output = generated_output("ai_json");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--json",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(value["ok"], true);
    assert_eq!(value["command"], "ai generate");
    assert_eq!(value["output_path"], output.display().to_string());
    cleanup(&output);
}

#[test]
fn test_ai_generate_prompt_without_api_key() {
    let result = Command::new(project_root().join("target/debug/rive-cli"))
        .args(["ai", "generate", "--prompt", "a red square"])
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("failed to run rive-cli");
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("OPENAI_API_KEY"));
}

#[test]
fn test_ai_no_prompt_or_template() {
    let result = cargo_run(&["ai", "generate"]);
    assert_failure(&result);
}

#[test]
fn test_ai_rejects_both_template_and_prompt() {
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--prompt",
        "a red square",
    ]);
    assert_failure(&result);
}

#[test]
fn test_ai_unknown_template_error() {
    let result = cargo_run(&["ai", "generate", "--template", "not-a-template"]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("unknown template"));
}

#[test]
fn test_ai_max_retries_flag_accepted() {
    let output = generated_output("ai_retries");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--max-retries",
        "5",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    cleanup(&output);
}

#[test]
fn test_ai_generate_with_repair_retries() {
    let output = generated_output("ai_repair");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--max-retries",
        "3",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    cleanup(&output);
}

#[test]
fn test_ai_repair_zero_retries_still_succeeds_valid_template() {
    let output = generated_output("ai_zero_retry");
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--max-retries",
        "0",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&result);
    cleanup(&output);
}

#[test]
fn test_ai_repair_dry_run_skips_repair() {
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--dry-run",
        "--max-retries",
        "0",
    ]);
    assert_success(&result);
}

#[test]
fn test_ai_dry_run_pipe_to_generate() {
    let scene = cargo_run(&["ai", "generate", "--template", "bounce", "--dry-run"]);
    assert_success(&scene);
    let root = std::env::temp_dir().join("rive_ai_pipe");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let input = root.join("scene.json");
    let output = root.join("out.riv");
    fs::write(&input, &scene.stdout).unwrap();
    let generated = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert_success(&generated);
    assert_success(&validate(&output));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_ai_generate_json_flag_in_help() {
    let result = cargo_run(&["ai", "generate", "--help"]);
    assert_success(&result);
    assert!(String::from_utf8_lossy(&result.stdout).contains("--json"));
}

#[test]
fn test_ai_lab_generates_report_and_artifacts() {
    let root = std::env::temp_dir().join("rive_ai_lab_e2e");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    let suite_path = root.join("suite.json");
    let suite_json = r#"{
      "suite_name": "e2e-suite",
      "suite_version": 1,
      "cases": [
        {
          "id": "bounce-case",
          "input_kind": "template",
          "input": "bounce",
          "expected_traits": ["has_animation"]
        }
      ]
    }"#;
    fs::write(&suite_path, suite_json).unwrap();

    let output_dir = root.join("runs");
    let result = cargo_run(&[
        "ai",
        "lab",
        "--suite",
        suite_path.to_str().unwrap(),
        "--output-dir",
        output_dir.to_str().unwrap(),
    ]);
    assert_success(&result);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let run_id_line = stdout
        .lines()
        .find(|line| line.starts_with("run_id="))
        .expect("missing run_id output");
    let run_id = run_id_line.trim_start_matches("run_id=");
    let run_dir = output_dir.join(run_id);
    assert!(run_dir.join("report.json").exists());
    assert!(run_dir.join("samples/bounce-case/scene.json").exists());
    assert!(run_dir.join("samples/bounce-case/output.riv").exists());
    assert!(run_dir.join("samples/bounce-case/validate.json").exists());
    assert!(run_dir.join("samples/bounce-case/inspect.json").exists());
    let report = read_json(&run_dir.join("report.json"));
    assert_eq!(report["case_count"], 1);
    assert_eq!(report["valid_count"], 1);
    assert_eq!(report["passed"], true);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_ai_lab_regression_flags_drift() {
    let root = std::env::temp_dir().join("rive_ai_lab_e2e_drift");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    let suite_path = root.join("suite.json");
    let suite_json = r#"{
      "suite_name": "e2e-drift-suite",
      "suite_version": 1,
      "cases": [
        {
          "id": "bounce-case",
          "input_kind": "template",
          "input": "bounce",
          "expected_traits": ["has_animation"]
        }
      ]
    }"#;
    fs::write(&suite_path, suite_json).unwrap();

    let baseline_path = root.join("baseline.json");
    let baseline_json = r#"{
      "suite_name": "e2e-drift-suite",
      "suite_version": 1,
      "case_hashes": {
        "bounce-case": "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
      }
    }"#;
    fs::write(&baseline_path, baseline_json).unwrap();

    let output_dir = root.join("runs");
    let result = cargo_run(&[
        "ai",
        "lab",
        "--suite",
        suite_path.to_str().unwrap(),
        "--output-dir",
        output_dir.to_str().unwrap(),
        "--baseline",
        baseline_path.to_str().unwrap(),
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("regression drift detected"));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_ai_lab_rejects_prompt_suite_with_template_provider() {
    let root = std::env::temp_dir().join("rive_ai_lab_prompt_provider");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let suite_path = root.join("suite.json");
    fs::write(
        &suite_path,
        r#"{
          "suite_name": "prompt-suite",
          "suite_version": 1,
          "cases": [{
            "id": "prompt-case",
            "input_kind": "prompt",
            "input": "a red square",
            "expected_traits": []
          }]
        }"#,
    )
    .unwrap();
    let result = cargo_run(&[
        "ai",
        "lab",
        "--suite",
        suite_path.to_str().unwrap(),
        "--provider",
        "template",
    ]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("prompt cases require"));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_ai_lab_rejects_duplicate_case_ids() {
    let root = std::env::temp_dir().join("rive_ai_lab_duplicate_cases");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let suite_path = root.join("suite.json");
    fs::write(
        &suite_path,
        r#"{
          "suite_name": "duplicate-suite",
          "suite_version": 1,
          "cases": [
            {"id":"same","input_kind":"template","input":"bounce"},
            {"id":"same","input_kind":"template","input":"spinner"}
          ]
        }"#,
    )
    .unwrap();
    let result = cargo_run(&["ai", "lab", "--suite", suite_path.to_str().unwrap()]);
    assert_failure(&result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("duplicate case id"));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_list_presets_flag() {
    let result = cargo_run(&["--list-presets"]);
    assert_success(&result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("square"));
}

#[test]
fn test_list_presets_json() {
    let result = cargo_run(&["--list-presets", "--json"]);
    assert_success(&result);
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert!(value.as_array().is_some());
}

#[test]
fn test_multiple_fixtures_validate() {
    for fixture in [
        "minimal.json",
        "shapes.json",
        "animation.json",
        "state_machine.json",
        "text.json",
    ] {
        let name = fixture.trim_end_matches(".json");
        let output = generate_fixture(fixture, &format!("multiple_{name}"));
        assert_success(&validate(&output));
        cleanup(&output);
    }
}

#[test]
fn test_new_templates_generate_and_validate() {
    let root = std::env::temp_dir().join("rive_cli_new_templates");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    for template in ["shape", "animated", "gradient", "spinner", "button", "multi"] {
        let scene = root.join(format!("{template}.json"));
        let output = root.join(format!("{template}.riv"));
        let scaffold = cargo_run(&["new", template, "-o", scene.to_str().unwrap()]);
        assert_success(&scaffold);
        let generate = cargo_run(&[
            "generate",
            scene.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ]);
        assert_success(&generate);
        assert_success(&validate(&output));
    }
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_showcases_generate_and_validate() {
    for entry in fs::read_dir(project_root().join("showcase")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }
        let output = temp_output(
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("showcase"),
        );
        cleanup(&output);
        let result = cargo_run(&[
            "generate",
            path.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ]);
        assert_success(&result);
        assert_success(&validate(&output));
        cleanup(&output);
    }
}

#[test]
fn test_showcase_riv_files_are_up_to_date() {
    for entry in fs::read_dir(project_root().join("showcase")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }
        let committed = path.with_extension("riv");
        if !committed.exists() {
            continue;
        }
        let output = temp_output(
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("showcase_sync"),
        );
        cleanup(&output);
        let result = cargo_run(&[
            "generate",
            path.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "--file-id",
            "0",
        ]);
        assert_success(&result);
        assert_eq!(fs::read(&output).unwrap(), fs::read(&committed).unwrap());
        cleanup(&output);
    }
}
