import copy
import json
from pathlib import Path


def replace(path, old, new):
    file = Path(path)
    text = file.read_text()
    if new in text:
        return
    assert text.count(old) == 1, (path, text.count(old))
    file.write_text(text.replace(old, new))


source = json.loads(Path('examples/authoring/model-blend-panel.v0.json').read_text())
literal = lambda value, unit='scalar': {'kind': 'literal', 'value': value, 'unit': unit}
poses = []
tracks = []
for side, y in [('left', 40), ('right', 120)]:
    for label, x in [('rest', 40), ('middle', 120), ('full', 200)]:
        identity = f'{side}-{label}'
        poses.append({'id': identity, 'targets': [{'target': f'{side}-panel', 'transform': {'x': literal(x, 'px'), 'y': literal(y, 'px')}}]})
        tracks.append({'id': f'{identity}-track', 'fps': 60, 'duration_frames': literal(1), 'keyframes': [{'frame': literal(0), 'pose': identity}, {'frame': literal(1), 'pose': identity}]})
source['motion'] = {'poses': poses, 'tracks': tracks}
chart = source['behavior']['statecharts'][0]
chart['inputs'] = [item for item in chart['inputs'] if item['kind'] == 'trigger']

def states(side):
    return [{'id': 'blending', 'blend': {'binding': f'{side}-model', 'stops': [{'motion': f'{side}-{label}-track', 'value': literal(value)} for label, value in [('rest', 0), ('middle', 50), ('full', 100)]]}}, {'id': 'resting', 'motion': f'{side}-rest-track'}]

chart['states'] = states('left')
chart['regions'] = [{'id': 'right', 'initial': 'blending', 'states': states('right'), 'transitions': copy.deepcopy(chart['transitions'])}]
Path('examples/authoring/model-blend-1d-panel.v0.json').write_text(json.dumps(source, indent=2) + '\n')

path = Path('tests/authoring_model_blend_1d_contract.rs')
if 'fn blend_sources_are_exclusive' not in path.read_text():
    path.write_text(path.read_text() + r'''
fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("valid model blend")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("binary compilation")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid blend");
    assert!(error.diagnostics.iter().any(|entry| entry.code == code && entry.path == path), "{error:?}");
}

#[test]
fn blend_sources_are_exclusive_and_reject_nulls_and_runtime_indices() {
    for source in [json!({}), json!({"input":"load","binding":"model-load"}),
        json!({"binding":null}), json!({"input":null}), json!({"binding":"model-load","input_id":0}),
        json!({"binding":"model-load","weight":50})] {
        let mut input = document();
        let stops = input["behavior"]["statecharts"][0]["states"][0]["blend"]["stops"].clone();
        let mut blend = source;
        blend["stops"] = stops;
        input["behavior"]["statecharts"][0]["states"][0]["blend"] = blend;
        assert_diagnostic(&input, "invalid_json", "$");
    }
}

#[test]
fn model_blend_errors_preserve_authored_reference_paths() {
    for (pointer, code, path) in [
        ("/behavior/statecharts/0/states/0/blend/binding", "unknown_behavior_binding", "$.behavior.statecharts[0].states[0].blend.binding"),
        ("/behavior/statecharts/0/states/0/blend/stops/0/motion", "unknown_behavior_motion", "$.behavior.statecharts[0].states[0].blend.stops[0].motion"),
        ("/behavior/bindings/0/model", "unknown_behavior_model", "$.behavior.bindings[0].model"),
        ("/behavior/bindings/0/property", "unknown_behavior_property", "$.behavior.bindings[0].property"),
    ] {
        let mut input = document();
        *input.pointer_mut(pointer).expect("reference") = json!("missing");
        assert_diagnostic(&input, code, path);
    }
    let mut input = document();
    input["behavior"]["models"][0]["properties"][0] = json!({"kind":"bool","id":"load","value":false});
    assert_diagnostic(&input, "invalid_blend_binding", "$.behavior.statecharts[0].states[0].blend.binding");
}

#[test]
fn model_blends_keep_stop_limits_and_emitted_float_ordering() {
    for length in [0, 1, 1001] {
        let mut input = document();
        let stops: Vec<_> = (0..length).map(|index| json!({"motion":"calm-track", "value":{"kind":"literal","value":index,"unit":"scalar"}})).collect();
        input["behavior"]["statecharts"][0]["states"][0]["blend"]["stops"] = json!(stops);
        assert_diagnostic(&input,"invalid_blend_stops","$.behavior.statecharts[0].states[0].blend.stops");
    }
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0]["blend"]["stops"][0]["value"]["value"] = json!(16_777_216);
    input["behavior"]["statecharts"][0]["states"][0]["blend"]["stops"][1]["value"]["value"] = json!(16_777_217);
    assert_diagnostic(&input,"invalid_blend_stop_order","$.behavior.statecharts[0].states[0].blend.stops[1].value");
    let mut input = document();
    input["parameters"] = json!({"maximum":{"value":100,"unit":"scalar"}});
    input["behavior"]["statecharts"][0]["states"][0]["blend"]["stops"][1]["value"] = json!({"kind":"parameter","name":"maximum"});
    let lowered = lower(&input);
    assert_eq!(lowered.scene["artboard"]["state_machines"][0]["layers"][0]["states"][1]["children"][1]["value"],100.0);
    compile(&lowered.scene);
}

#[test]
fn a_region_only_blend_binding_is_discovered_and_mapped() {
    let mut input = document();
    let state = input["behavior"]["statecharts"][0]["states"][0].clone();
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["states"][0] = json!({"id":"reading","motion":"calm-track"});
    chart["regions"] = json!([{"id":"secondary","initial":"reading","states":[state]}]);
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"].as_array().expect("inputs").len(),2);
    assert_eq!(machine["layers"][1]["states"][1]["input"],machine["inputs"][0]["name"]);
    let source = lowered.source_map.entries.iter().find(|entry|entry.authored_id=="meter/secondary/reading").expect("region source");
    assert_eq!(source.scene_paths,["/artboard/state_machines/0/layers/1/states/1"]);
    compile(&lowered.scene);
    input["behavior"]["statecharts"][0]["regions"][0]["states"][0]["blend"]["binding"] = json!("missing");
    assert_diagnostic(&input,"unknown_behavior_binding","$.behavior.statecharts[0].regions[0].states[0].blend.binding");
}

#[test]
fn one_binding_is_shared_across_one_dimensional_direct_and_transition_consumers() {
    let mut input = document();
    let state = input["behavior"]["statecharts"][0]["states"][0].clone();
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["states"].as_array_mut().expect("states").push(json!({"id":"resting","motion":"calm-track"}));
    chart["transitions"] = json!([{"id":"reset","from":"reading","to":"resting","when":{"binding":"model-load","compare":"greater_or_equal","value":{"kind":"literal","value":60,"unit":"scalar"}}}]);
    chart["regions"] = json!([
        {"id":"secondary","initial":"reading","states":[state]},
        {"id":"direct","initial":"reading","states":[{"id":"reading","direct_blend":{"motions":[{"motion":"surge-track","binding":"model-load"}]}}]}
    ]);
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"].as_array().expect("inputs").len(),2);
    for layer in [0,1] {
        assert_eq!(machine["layers"][layer]["states"][1]["input"],machine["inputs"][0]["name"]);
    }
    assert_eq!(machine["layers"][2]["states"][1]["children"][0]["input_id"],0);
    let parsed = parse_riv(&compile(&lowered.scene),&InspectFilter::default()).expect("binary");
    assert_eq!(parsed.objects.iter().filter(|object|object.type_key==type_keys::DATA_BIND_CONTEXT).count(),4);
    assert_eq!(parsed.objects.iter().filter(|object|object.type_key==type_keys::BLEND_STATE_1D_VIEW_MODEL).count(),2);
    let source = lowered.source_map.entries.iter().find(|entry|entry.authored_id=="model-load").expect("binding source");
    assert_eq!(source.scene_paths,["/artboard/state_machines/0/inputs/0"]);
}

#[test]
fn model_blend_sources_keep_chart_local_offsets_and_deterministic_maps() {
    let mut input = document();
    let mut other = input["behavior"]["statecharts"][0].clone();
    other["id"] = json!("other");
    input["behavior"]["statecharts"].as_array_mut().expect("charts").push(other);
    input["behavior"]["models"][0]["properties"].as_array_mut().expect("properties").push(json!({"kind":"bool","id":"enabled","value":false}));
    input["behavior"]["bindings"].as_array_mut().expect("bindings").insert(0,json!({"id":"enabled-model","model":"model","property":"enabled"}));
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["states"].as_array_mut().expect("states").push(json!({"id":"resting","motion":"calm-track"}));
    chart["transitions"] = json!([{"id":"reset","from":"reading","to":"resting","when":{"binding":"enabled-model","equals":true}}]);
    let lowered = lower(&input);
    let machines = &lowered.scene["artboard"]["state_machines"];
    for (chart,index) in [(0,1),(1,0)] {
        assert_eq!(machines[chart]["layers"][0]["states"][1]["input"],machines[chart]["inputs"][index]["name"]);
    }
    let source = lowered.source_map.entries.iter().find(|entry|entry.authored_id=="model-load").expect("binding source");
    assert_eq!(source.scene_paths,["/artboard/state_machines/0/inputs/1","/artboard/state_machines/1/inputs/0"]);
    for entry in &lowered.source_map.entries {
        for path in &entry.scene_paths { assert!(lowered.scene.pointer(path).is_some(),"{path}"); }
    }
    assert_eq!(lowered,lower(&input));
    compile(&lowered.scene);
}

#[test]
fn native_model_blend_paths_encode_nonzero_models_and_multibyte_properties() {
    use rive_cli::builder::{SceneSpec, build_scene};
    use rive_cli::objects::core::PropertyValue;
    let mut input = document();
    let load = input["behavior"]["models"][0]["properties"][0].clone();
    let mut properties: Vec<_> = (0..128).map(|index|json!({"kind":"bool","id":format!("unused-{index}"),"value":false})).collect();
    properties.push(load);
    input["behavior"]["models"][0]["properties"] = json!(properties);
    input["behavior"]["models"].as_array_mut().expect("models").insert(0,json!({"id":"earlier","properties":[{"kind":"bool","id":"unused","value":false}]}));
    let mut scene = lower(&input).scene;
    scene["artboard"]["state_machines"][0]["layers"][0]["states"][1].as_object_mut().expect("state").remove("input");
    scene["artboard"]["state_machines"][0]["layers"][0]["states"][1]["input_id"] = json!(0);
    let scene: SceneSpec = serde_json::from_value(scene).expect("canonical scene");
    let objects = build_scene(&scene,None).expect("scene objects");
    let context = objects.iter().position(|object|object.type_key()==type_keys::DATA_BIND_CONTEXT).expect("context");
    assert_eq!(objects[context+1].type_key(),type_keys::BLEND_STATE_1D_VIEW_MODEL);
    assert!(objects[context].properties().iter().any(|field|field.key==property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS && field.value==PropertyValue::Bytes(vec![1,128,1])));
    compile_scene(&scene,None,0).expect("multibyte binding compiles");
}

#[test]
fn an_unbound_one_dimensional_blend_keeps_its_input_state() {
    let source: Value = serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json")).expect("input example");
    let lowered = lower(&source);
    let parsed = parse_riv(&compile(&lowered.scene),&InspectFilter::default()).expect("binary");
    assert_eq!(parsed.objects.iter().filter(|object|object.type_key==type_keys::BLEND_STATE_1D_INPUT).count(),1);
    assert!(!parsed.objects.iter().any(|object|object.type_key==type_keys::DATA_BIND_CONTEXT || object.type_key==type_keys::BLEND_STATE_1D_VIEW_MODEL));
}

#[test]
fn a_failed_incremental_edit_preserves_model_blend_identity() {
    use rive_cli::authoring::{AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec, AuthoringTarget, apply_operations, lower_authoring};
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed source");
    let snapshot = serde_json::to_value(&spec).expect("snapshot");
    let before = lower_authoring(&spec).expect("initial lowering");
    let mut replacement = input["behavior"]["statecharts"][0].clone();
    replacement["states"][0]["blend"]["binding"] = json!("missing");
    let operations = [AuthoringOperation::Remove{target:AuthoringTarget::BehaviorStatechart{target_id:"meter".to_string()}},AuthoringOperation::Insert{entity:AuthoringEntity::BehaviorStatechart(serde_json::from_value(replacement).expect("replacement")),placement:AuthoringPlacement::Into{container:AuthoringContainer::BehaviorStatecharts}}];
    let error = apply_operations(&spec,&operations).expect_err("invalid source must roll back");
    assert!(error.diagnostics.iter().any(|entry|entry.code=="unknown_behavior_binding"));
    assert_eq!(serde_json::to_value(&spec).expect("unchanged"),snapshot);
    assert_eq!(lower_authoring(&spec).expect("still valid"),before);
}

#[test]
fn the_model_bound_one_dimensional_panel_compiles_deterministically() {
    let source: Value = serde_json::from_str(include_str!("../examples/authoring/model-blend-1d-panel.v0.json")).expect("model panel");
    let first = lower(&source);
    let second = lower(&source);
    assert_eq!(first,second);
    assert_eq!(compile(&first.scene),compile(&second.scene));
}
''')

replace('cairn.blueprint', '                "./tests/authoring_model_blend_contract.rs",', '                "./tests/authoring_model_blend_1d_contract.rs",\n                "./tests/authoring_model_blend_contract.rs",')
path = 'tests/playwright/authoring-direct-blend-runtime.js'
replace(path, '''const MODE = MODEL_BOUND ? "model-blend" : "direct-blend";
const FIXTURE = MODEL_BOUND ? "authoring_model_blend" : "authoring_direct_blend";''', '''const ONE_DIMENSIONAL = process.argv.includes("--one-dimensional");
assert.ok(!ONE_DIMENSIONAL || MODEL_BOUND, "one-dimensional mode requires --model-bound");
const MODE = ONE_DIMENSIONAL ? "model-blend-1d" : MODEL_BOUND ? "model-blend" : "direct-blend";
const FIXTURE = `authoring_${MODE.replaceAll("-", "_")}`;''')
replace(path, '''    assert.ok(Math.abs(measured[side].centerX - x) <= POSITION_TOLERANCE,
      `${label}: expected ${side} at ${x}, got ${JSON.stringify(measured)}`);''', '''    const position = measured[side].centerX;
    const matches = Array.isArray(x)
      ? position > x[0] && position < x[1]
      : Math.abs(position - x) <= POSITION_TOLERANCE;
    assert.ok(matches, `${label}: expected ${side} at ${JSON.stringify(x)}, got ${JSON.stringify(measured)}`);''')
replace(path, '''      ["zero", 0, 0, 40, 40],
      ["left-half",''', '''      ["zero", 0, 0, 40, 40],
      ...(ONE_DIMENSIONAL ? [
        ["between-stops", 25, 75, [41, 119], [121, 199]],
        ["reverse-between-stops", 75, 25, [121, 199], [41, 119]],
      ] : []),
      ["left-half",''')
replace(path, '''    await page.evaluate(() => window.__DIRECT_BLEND.resume.fire());
    samples.push(await sample(page, "resume-state", { left: 200, right: 200 }));''', '''    if (ONE_DIMENSIONAL) {
      await page.evaluate(() => {
        window.__DIRECT_BLEND.left.value = 25;
        window.__DIRECT_BLEND.right.value = 75;
      });
      samples.push(await sample(page, "model-change-while-resting", { left: 40, right: 40 }));
    }
    await page.evaluate(() => window.__DIRECT_BLEND.resume.fire());
    samples.push(await sample(page, "resume-state", ONE_DIMENSIONAL
      ? { left: [41, 119], right: [121, 199] }
      : { left: 200, right: 200 }));''')
replace('.github/workflows/ci.yml', '          node tests/playwright/authoring-direct-blend-runtime.js --model-bound\n', '          node tests/playwright/authoring-direct-blend-runtime.js --model-bound\n          node tests/playwright/authoring-direct-blend-runtime.js --model-bound --one-dimensional\n')
