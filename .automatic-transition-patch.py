from pathlib import Path

path = Path("src/authoring/spec.rs")
text = path.read_text()
old = """#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorAlwaysGuardSpec {
    Always,
}
"""
new = """#[derive(Debug, Clone, Copy, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorAlwaysGuardSpec {
    Always,
}

impl<'de> Deserialize<'de> for BehaviorAlwaysGuardSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "always" => Ok(Self::Always),
            _ => Err(serde::de::Error::unknown_variant(&value, &["always"])),
        }
    }
}
"""
assert text.count(old) == 1
path.write_text(text.replace(old, new))
path = Path("tests/authoring_automatic_transition_contract.rs")
text = path.read_text()
old = '        json!({"always":true}),'
new = old + '\n        json!({"always":null}),\n        json!({"always":[]}),'
assert text.count(old) == 1
path.write_text(text.replace(old, new))
Path("tests/authoring_automatic_guard_json_contract.rs").unlink()
Path(__file__).unlink()
