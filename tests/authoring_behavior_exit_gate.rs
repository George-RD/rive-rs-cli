mod support;

use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};
use support::assert_builds;

const SHOWCASE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/authoring/complex-interactive-showcase.v0.json"
));

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("complex interactive showcase must lower")
}

#[test]
fn complex_interactive_showcase_matches_direct_scene_spec() {
    let authored: Value = serde_json::from_str(SHOWCASE).expect("showcase must be valid JSON");
    let statechart = &authored["behavior"]["statecharts"][0];

    assert_eq!(statechart["inputs"].as_array().map(Vec::len), Some(2));
    assert_eq!(statechart["events"].as_array().map(Vec::len), Some(1));
    assert_eq!(statechart["listeners"].as_array().map(Vec::len), Some(3));
    assert_eq!(statechart["states"].as_array().map(Vec::len), Some(3));
    assert_eq!(statechart["transitions"].as_array().map(Vec::len), Some(4));
    assert!(authored["behavior"].get("raw_state_machines").is_none());
    assert!(authored["motion"].get("raw_animations").is_none());

    let actual = lower(&authored);

    let mut behaviorless = authored.clone();
    behaviorless["behavior"] = json!({});
    let mut expected = lower(&behaviorless).scene;
    expected["artboard"]["children"]
        .as_array_mut()
        .expect("canonical artboard children")
        .push(json!({
            "type": "event",
            "name": "auth__showcase__flow__reset__event"
        }));
    expected["artboard"]["state_machines"] = json!([
        {
            "name": "auth__showcase__flow__state_machine",
            "inputs": [
                {
                    "type": "bool",
                    "name": "auth__showcase__flow__selected__input",
                    "value": false
                },
                {
                    "type": "bool",
                    "name": "auth__showcase__flow__approved__input",
                    "value": false
                }
            ],
            "listeners": [
                {
                    "target": "auth__showcase__card__shape",
                    "listener_type": "click",
                    "actions": [
                        {
                            "type": "bool_change",
                            "input": "auth__showcase__flow__selected__input",
                            "value": true
                        }
                    ]
                },
                {
                    "target": "auth__showcase__card__shape",
                    "listener_type": "up",
                    "actions": [
                        {
                            "type": "bool_change",
                            "input": "auth__showcase__flow__approved__input",
                            "value": true
                        }
                    ]
                },
                {
                    "target": "auth__showcase__flow__reset__event",
                    "listener_type": "event",
                    "actions": [
                        {
                            "type": "bool_change",
                            "input": "auth__showcase__flow__selected__input",
                            "value": false
                        },
                        {
                            "type": "bool_change",
                            "input": "auth__showcase__flow__approved__input",
                            "value": false
                        }
                    ]
                }
            ],
            "layers": [
                {
                    "states": [
                        { "type": "entry" },
                        {
                            "type": "animation",
                            "animation": "auth__showcase__queuedtrack__animation"
                        },
                        {
                            "type": "animation",
                            "animation": "auth__showcase__focusedtrack__animation"
                        },
                        {
                            "type": "animation",
                            "animation": "auth__showcase__approvedtrack__animation"
                        },
                        { "type": "exit" }
                    ],
                    "transitions": [
                        { "from": 0, "to": 1 },
                        {
                            "from": 1,
                            "to": 2,
                            "conditions": [
                                {
                                    "input": "auth__showcase__flow__selected__input",
                                    "value": true
                                }
                            ]
                        },
                        {
                            "from": 2,
                            "to": 3,
                            "conditions": [
                                {
                                    "input": "auth__showcase__flow__approved__input",
                                    "value": true
                                }
                            ]
                        },
                        {
                            "from": 3,
                            "to": 2,
                            "conditions": [
                                {
                                    "input": "auth__showcase__flow__approved__input",
                                    "value": false
                                }
                            ]
                        },
                        {
                            "from": 2,
                            "to": 1,
                            "conditions": [
                                {
                                    "input": "auth__showcase__flow__selected__input",
                                    "value": false
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    ]);

    assert_eq!(actual.scene, expected);

    for expected_id in [
        "flow",
        "flow/approved",
        "flow/approved-state",
        "flow/approve",
        "flow/focused-state",
        "flow/focus",
        "flow/reset",
        "flow/reset-listener",
        "flow/select",
        "flow/selected",
        "flow/unapprove",
        "flow/unselect",
    ] {
        assert!(
            actual
                .source_map
                .entries
                .iter()
                .any(|entry| entry.authored_id == expected_id),
            "source map is missing {expected_id}"
        );
    }

    assert_builds(actual.scene);
}
