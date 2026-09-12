
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
                    artboards[0]["children"].as_array_mut().expect("children").push(invalid_child);
                }
                let scene = serde_json::from_value(json!({
                    "scene_format_version": 1,
                    "artboards": artboards
                })).expect("syntactically valid nested asset scene");
                let error = compile_scene_with_assets(&scene, options(), &NoReads)
                    .expect_err("invalid placement must fail before caller code runs");
                assert_eq!(error.code(), "invalid-scene");
                assert!(error.to_string().contains("NestedAsset"), "{error}");
            }
        }
    }
}
