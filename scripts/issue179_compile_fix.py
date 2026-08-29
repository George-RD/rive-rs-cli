from pathlib import Path


path = Path("src/builder/scene.rs")
text = path.read_text()

is_on_old = '''                    inputs: Some(vec![InputSpec::Bool {
                        name: "is_on".to_string(),
                        value: false,
                    }]),'''
is_on_new = '''                    inputs: Some(vec![InputSpec::Bool {
                        name: "is_on".to_string(),
                        value: false,
                        view_model_binding: None,
                    }]),'''
if text.count(is_on_old) != 1:
    raise SystemExit(f"expected one is_on bool initializer, found {text.count(is_on_old)}")
text = text.replace(is_on_old, is_on_new, 1)

enabled_old = '''                    inputs: Some(vec![InputSpec::Bool {
                        name: "enabled".to_string(),
                        value: false,
                    }]),'''
enabled_new = '''                    inputs: Some(vec![InputSpec::Bool {
                        name: "enabled".to_string(),
                        value: false,
                        view_model_binding: None,
                    }]),'''
if text.count(enabled_old) != 2:
    raise SystemExit(f"expected two enabled bool initializers, found {text.count(enabled_old)}")
text = text.replace(enabled_old, enabled_new)

path.write_text(text)
print("patched three InputSpec::Bool unit-test initializers")
