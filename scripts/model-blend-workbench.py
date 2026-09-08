from pathlib import Path
import json

if 'fn model_binding_sources_are_exclusive' in Path('tests/authoring_model_blend_contract.rs').read_text():
    raise SystemExit(0)


def replace(path, before, after):
    file = Path(path)
    text = file.read_text()
    assert text.count(before) == 1, (path, before[:80])
    file.write_text(text.replace(before, after))


file = Path('tests/authoring_model_blend_contract.rs')
file.write_text(file.read_text() + '''

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("valid bound blend")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("binary compilation")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid bound blend");
    assert!(error.diagnostics.iter().any(|entry| entry.code == code && entry.path == path), "{error:?}");
}

#[test]
fn model_binding_sources_are_exclusive_and_reject_runtime_indices() {
    for child in [
        json!({"motion":"left-track"}),
        json!({"motion":"left-track", "input":"left-weight", "binding":"left-model"}),
        json!({"motion":"left-track", "binding":null}),
        json!({"motion":"left-track", "binding":"left-model", "input_id":0}),
        json!({"motion":"left-track", "binding":"left-model", "weight":50}),
    ] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][1] = child;
        assert_diagnostic(&input, "invalid_json", "$");
    }
}

#[test]
fn model_blend_reference_errors_keep_authored_paths() {
    let child = "/behavior/statecharts/0/states/0/direct_blend/motions/1";
    for (pointer, path, code) in [
        (format!("{child}/binding"), "$.behavior.statecharts[0].states[0].direct_blend.motions[1].binding", "unknown_behavior_binding"),
        (format!("{child}/motion"), "$.behavior.statecharts[0].states[0].direct_blend.motions[1].motion", "unknown_behavior_motion"),
        ("/behavior/bindings/0/model".to_string(), "$.behavior.bindings[0].model", "unknown_behavior_model"),
        ("/behavior/bindings/0/property".to_string(), "$.behavior.bindings[0].property", "unknown_behavior_property"),
    ] {
        let mut input = document();
        *input.pointer_mut(&pointer).expect("reference") = json!("missing");
        assert_diagnostic(&input, code, path);
    }
    let mut input = document();
    input["behavior"]["models"][0]["properties"][0] = json!({"kind":"bool", "id":"left", "value":false});
    assert_diagnostic(&input, "invalid_blend_binding", "$.behavior.statecharts[0].states[0].direct_blend.motions[1].binding");
}

#[test]
fn a_region_only_model_binding_is_emitted_and_resolves_the_region_path() {
    let mut input = document();
    let state = input["behavior"]["statecharts"][0]["states"][0].clone();
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["states"][0] = json!({"id":"blending", "motion":"rest-track"});
    chart["regions"] = json!([{"id":"secondary", "initial":"blending", "states":[state]}]);
    let output = lower(&input);
    let machine = &output.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"].as_array().expect("inputs").len(), 6);
    assert_eq!(machine["layers"][1]["states"][1]["children"][1]["input_id"], 0);
    let source = output.source_map.entries.iter().find(|entry| entry.authored_id == "panel/secondary/blending").expect("region source");
    assert_eq!(source.scene_paths, ["/artboard/state_machines/0/layers/1/states/1"]);
    compile(&output.scene);
    input["behavior"]["models"][0]["properties"][0] = json!({"kind":"bool", "id":"left", "value":false});
    assert_diagnostic(&input, "invalid_blend_binding", "$.behavior.statecharts[0].regions[0].states[0].direct_blend.motions[1].binding");
}

#[test]
fn transition_root_and_region_consumers_share_one_synthesized_input() {
    let mut input = document();
    let state = input["behavior"]["statecharts"][0]["states"][0].clone();
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["regions"] = json!([{"id":"secondary", "initial":"blending", "states":[state]}]);
    chart["transitions"][0]["when"] = json!({"binding":"left-model", "compare":"greater_or_equal", "value":{"kind":"literal", "value":60, "unit":"scalar"}});
    let output = lower(&input);
    let machine = &output.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"].as_array().expect("inputs").len(), 6);
    for layer in [0, 1] {
        assert_eq!(machine["layers"][layer]["states"][1]["children"][1]["input_id"], 0);
    }
    let source = output.source_map.entries.iter().find(|entry| entry.authored_id == "left-model").expect("binding source");
    assert_eq!(source.scene_paths, ["/artboard/state_machines/0/inputs/0"]);
    let parsed = parse_riv(&compile(&output.scene), &InspectFilter::default()).expect("encoded scene");
    assert_eq!(parsed.objects.iter().filter(|object| object.type_key == type_keys::DATA_BIND_CONTEXT).count(), 3);
}

#[test]
fn shared_model_bindings_resolve_chart_local_input_offsets() {
    let mut input = document();
    let mut second = input["behavior"]["statecharts"][0].clone();
    second["id"] = json!("other-panel");
    input["behavior"]["statecharts"].as_array_mut().expect("charts").push(second);
    input["behavior"]["models"][0]["properties"].as_array_mut().expect("properties").push(json!({"kind":"bool", "id":"enabled", "value":false}));
    input["behavior"]["bindings"].as_array_mut().expect("bindings").insert(0, json!({"id":"enabled-model", "model":"weights", "property":"enabled"}));
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({"binding":"enabled-model", "equals":true});
    let output = lower(&input);
    let machines = &output.scene["artboard"]["state_machines"];
    assert_eq!(machines[0]["layers"][0]["states"][1]["children"][1]["input_id"], 1);
    assert_eq!(machines[1]["layers"][0]["states"][1]["children"][1]["input_id"], 0);
    let source = output.source_map.entries.iter().find(|entry| entry.authored_id == "left-model").expect("shared source");
    assert_eq!(source.scene_paths, ["/artboard/state_machines/0/inputs/1", "/artboard/state_machines/1/inputs/0"]);
    for entry in &output.source_map.entries {
        for pointer in &entry.scene_paths {
            assert!(output.scene.pointer(pointer).is_some(), "{pointer}");
        }
    }
    compile(&output.scene);
    assert_eq!(output, lower(&input));
}

#[test]
fn native_blend_binding_counts_model_properties_and_precedes_its_consumer() {
    use rive_cli::builder::build_scene;
    use rive_cli::objects::core::PropertyValue;
    let mut input = document();
    let left = input["behavior"]["models"][0]["properties"][0].clone();
    let mut properties: Vec<_> = (0..128).map(|index| json!({"kind":"bool", "id":format!("unused-{index}"), "value":false})).collect();
    properties.push(left);
    input["behavior"]["models"][0]["properties"] = json!(properties);
    input["behavior"]["models"].as_array_mut().expect("models").insert(0, json!({"id":"earlier", "properties":[{"kind":"bool", "id":"unused", "value":false}]}));
    let scene: SceneSpec = serde_json::from_value(lower(&input).scene).expect("canonical scene");
    let objects = build_scene(&scene, None).expect("build scene");
    let context = objects.iter().position(|object| object.type_key() == type_keys::DATA_BIND_CONTEXT).expect("binding context");
    assert_eq!(objects[context - 1].type_key(), type_keys::BINDABLE_PROPERTY_NUMBER);
    assert_eq!(objects[context + 1].type_key(), type_keys::BLEND_ANIMATION_DIRECT);
    assert!(objects[context].properties().iter().any(|field| field.key == property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS && field.value == PropertyValue::Bytes(vec![1, 128, 1])));
    assert!(objects[context - 1].properties().iter().any(|field| field.key == property_keys::BINDABLE_PROPERTY_NUMBER_VALUE && field.value == PropertyValue::Float(25.0)));
    compile_scene(&scene, None, 0).expect("multibyte model binding compiles");
}

#[test]
fn an_explicit_canonical_fixed_weight_is_not_reinterpreted_as_a_model_binding() {
    let mut scene = lower(&document()).scene;
    let child = &mut scene["artboard"]["state_machines"][0]["layers"][0]["states"][1]["children"][1];
    child["blend_source"] = json!(1);
    child["mix_value"] = json!(40);
    let parsed = parse_riv(&compile(&scene), &InspectFilter::default()).expect("encoded fixed blend");
    assert!(!parsed.objects.iter().any(|object| object.type_key == type_keys::DATA_BIND_CONTEXT));
    let child = parsed.objects.iter().filter(|object| object.type_key == type_keys::BLEND_ANIMATION_DIRECT).nth(1).expect("fixed child");
    assert!(child.properties.iter().any(|field| field.key == property_keys::BLEND_ANIMATION_DIRECT_BLEND_SOURCE && field.value == PropertyValueRead::UInt(1)));
    assert!(child.properties.iter().any(|field| field.key == property_keys::BLEND_ANIMATION_DIRECT_MIX_VALUE && field.value == PropertyValueRead::Float(40.0)));
}

#[test]
fn input_only_direct_blends_retain_their_pre_model_binding_binary() {
    use sha2::{Digest, Sha256};
    let input: Value = serde_json::from_str(include_str!("../examples/authoring/direct-blend-panel.v0.json")).expect("input-driven example");
    let bytes = compile(&lower(&input).scene);
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), "5f2ae61031eb5c0474e66d87f66c30678825b467fa9b15e82504463074b55f99");
}

#[test]
fn a_failed_statechart_edit_preserves_model_blend_source_identity() {
    use rive_cli::authoring::{AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec, AuthoringTarget, apply_operations, lower_authoring};
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed source");
    let snapshot = serde_json::to_value(&spec).expect("snapshot");
    let before = lower_authoring(&spec).expect("initial lowered source");
    let mut replacement = input["behavior"]["statecharts"][0].clone();
    replacement["states"][0]["direct_blend"]["motions"][1]["binding"] = json!("missing");
    let operations = [
        AuthoringOperation::Remove { target: AuthoringTarget::BehaviorStatechart { target_id: "panel".to_string() } },
        AuthoringOperation::Insert { entity: AuthoringEntity::BehaviorStatechart(serde_json::from_value(replacement).expect("replacement")), placement: AuthoringPlacement::Into { container: AuthoringContainer::BehaviorStatecharts } },
    ];
    let error = apply_operations(&spec, &operations).expect_err("invalid model source must roll back");
    assert!(error.diagnostics.iter().any(|entry| entry.code == "unknown_behavior_binding"));
    assert_eq!(serde_json::to_value(&spec).expect("unchanged"), snapshot);
    assert_eq!(lower_authoring(&spec).expect("still valid"), before);
}
''')

replace('tests/authoring_direct_blend_contract.rs', '''    assert_eq!(motion["additionalProperties"], false);
    assert_eq!(motion["required"], json!(["motion", "input"]));''', '''    let variants = motion["anyOf"].as_array().expect("exclusive weight sources");
    assert_eq!(variants.len(), 2);
    assert_eq!(variants[0]["additionalProperties"], false);
    assert_eq!(variants[0]["required"], json!(["motion", "input"]));
    assert_eq!(variants[1]["additionalProperties"], false);
    assert_eq!(variants[1]["required"], json!(["motion", "binding"]));''')

example = json.loads(Path('examples/authoring/direct-blend-panel.v0.json').read_text())
example['behavior']['models'] = [{'id': 'weights', 'properties': [
    {'kind': 'number', 'id': side, 'value': {'kind': 'literal', 'value': 0, 'unit': 'scalar'}}
    for side in ['left', 'right']
]}]
example['behavior']['bindings'] = [
    {'id': f'{side}-model', 'model': 'weights', 'property': side}
    for side in ['left', 'right']
]
chart = example['behavior']['statecharts'][0]
chart['inputs'] = [value for value in chart['inputs'] if value['id'] not in ['left-weight', 'right-weight']]
for index, side in enumerate(['left', 'right'], 1):
    chart['states'][0]['direct_blend']['motions'][index] = {'motion': f'{side}-track', 'binding': f'{side}-model'}
Path('examples/authoring/model-blend-panel.v0.json').write_text(json.dumps(example, indent=2) + '\n')

p = 'tests/playwright/authoring-direct-blend-runtime.js'
replace(p, '''const FIXTURE = "authoring_direct_blend";
const DIRECTORY = path.join(ROOT, "target/playwright-behavior/direct-blend");
const SOURCE = path.join(ROOT, "examples/authoring/direct-blend-panel.v0.json");''', '''const MODEL_BOUND = process.argv.includes("--model-bound");
const MODE = MODEL_BOUND ? "model-blend" : "direct-blend";
const FIXTURE = MODEL_BOUND ? "authoring_model_blend" : "authoring_direct_blend";
const DIRECTORY = path.join(ROOT, "target/playwright-behavior", MODE);
const SOURCE = path.join(ROOT, "examples/authoring", `${MODE}-panel.v0.json`);''')
replace(p, '''    left: name("panel/left-weight"),
    right: name("panel/right-weight"),''', '''    left: name(MODEL_BOUND ? "left-model" : "panel/left-weight"),
    right: name(MODEL_BOUND ? "right-model" : "panel/right-weight"),
    model: MODEL_BOUND ? name("weights") : null,
    leftProperty: MODEL_BOUND ? name("weights/left") : null,
    rightProperty: MODEL_BOUND ? name("weights/right") : null,''')
replace(p, '''        stateMachines: [plan.machine],''', '''        autoBind: false, stateMachines: [plan.machine],''')
replace(p, '''    window.__DIRECT_BLEND = {
      runtime, left: find(plan.left), right: find(plan.right),
      reset: find(plan.reset), resume: find(plan.resume),
    };''', '''    const leftInput = find(plan.left);
    const rightInput = find(plan.right);
    let instance = null;
    let left = leftInput;
    let right = rightInput;
    if (plan.model) {
      const model = runtime.viewModelByName(plan.model);
      if (!model) throw new Error(`missing model ${plan.model}`);
      instance = model.instance();
      if (!instance) throw new Error("missing model instance");
      left = instance.number(plan.leftProperty);
      right = instance.number(plan.rightProperty);
      if (!left || !right) throw new Error("missing model weight properties");
      left.value = 0;
      right.value = 0;
      runtime.bindViewModelInstance(instance);
    }
    window.__DIRECT_BLEND = {
      runtime, instance, left, right, leftInput, rightInput,
      reset: find(plan.reset), resume: find(plan.resume),
    };''')
replace(p, '''      rightWeight: window.__DIRECT_BLEND.right.value,''', '''      rightWeight: window.__DIRECT_BLEND.right.value,
      leftInput: window.__DIRECT_BLEND.leftInput.value,
      rightInput: window.__DIRECT_BLEND.rightInput.value,''')
replace(p, '''    await mount(page, plan);
    for (const [label''', '''    await mount(page, plan);
    if (MODEL_BOUND) {
      samples.push(await sample(page, "initial-model", { left: 40, right: 40 }));
      await page.evaluate(() => {
        window.__DIRECT_BLEND.leftInput.value = 90;
        window.__DIRECT_BLEND.rightInput.value = 10;
      });
      samples.push(await sample(page, "input-only", { left: 40, right: 40 }));
      assert.equal(samples.at(-1).leftWeight, 0);
      assert.equal(samples.at(-1).rightWeight, 0);
    }
    for (const [label''')
replace(p, '''    assert.deepEqual(errors, []);
    const evidence = {''', '''    assert.deepEqual(errors, []);
    if (MODEL_BOUND) {
      for (const sample of samples.slice(1)) {
        assert.equal(sample.leftInput, 90, `${sample.label}: model must not mirror input`);
        assert.equal(sample.rightInput, 10, `${sample.label}: model must not mirror input`);
      }
    }
    const evidence = {''')
replace(p, '''    console.log("direct blend weights controlled independent motions, clamped, reversed, exited and resumed");''', '''    console.log(`${MODE}: independent weights clamped, reversed, exited and resumed${MODEL_BOUND ? "; model changes, not synthesized inputs, controlled the result" : ""}`);''')
