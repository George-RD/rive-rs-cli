use rive_cli::ai::RepairEngine;
use serde_json::json;

#[test]
fn duplicate_name_repair_keeps_existing_reference_on_first_object() {
    let input = json!({
        "artboard": {
            "name": "RepairReference",
            "width": 500,
            "height": 500,
            "children": [
                { "type": "shape", "name": "foo", "children": [] },
                { "type": "shape", "name": "foo", "children": [] },
                { "type": "shape", "name": "foo", "children": [] }
            ],
            "animations": [
                {
                    "name": "move",
                    "fps": 60,
                    "duration": 2,
                    "keyframes": [
                        {
                            "object": "foo",
                            "property": "x",
                            "frames": [
                                { "frame": 0, "value": 0.0 },
                                { "frame": 1, "value": 10.0 }
                            ]
                        }
                    ]
                }
            ]
        }
    });

    let result = RepairEngine::default()
        .repair(input, 0)
        .expect("duplicate names should be repaired");
    let children = result.scene_json["artboard"]["children"]
        .as_array()
        .expect("artboard children");

    assert_eq!(children[0]["name"], "foo");
    assert_eq!(children[1]["name"], "foo_2");
    assert_eq!(children[2]["name"], "foo_3");
    assert_eq!(
        result.scene_json["artboard"]["animations"][0]["keyframes"][0]["object"], "foo",
        "an ambiguous pre-repair reference must remain on the first object that retains the original name"
    );
}
