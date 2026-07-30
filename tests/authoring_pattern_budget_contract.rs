use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn rectangle(index: usize) -> Value {
    json!({
        "kind": "rectangle",
        "id": format!("tile-{index}"),
        "width": literal(16.0, "px"),
        "height": literal(12.0, "px"),
        "fill": "#2563EB"
    })
}

fn document() -> String {
    let children = (0..101).map(rectangle).collect::<Vec<_>>();
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 220.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "radial",
                    "id": "orbit",
                    "copies": 100,
                    "radius": literal(20.0, "px"),
                    "start_angle": literal(0.0, "degrees"),
                    "angle_step": literal(3.6, "degrees"),
                    "item": {
                        "kind": "group",
                        "id": "bundle",
                        "children": children
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

#[test]
fn pattern_budget_counts_descendants_generated_by_radial_copies() {
    let error = lower_authoring_json(&document())
        .expect_err("pattern descendants must share the generated-item budget");
    assert!(
        error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "pattern_expansion_node_limit"
                && diagnostic.path == "$.visual.nodes[0].copies"
        }),
        "missing descendant expansion diagnostic: {:#?}",
        error.diagnostics
    );
}
