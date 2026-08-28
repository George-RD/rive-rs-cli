from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one match, found {count}: {old[:80]!r}")
    file.write_text(text.replace(old, new, 1))


replace_once(
    "src/objects/core.rs",
    "    pub const DATA_BIND: u16 = 446;\n",
    "    pub const DATA_BIND: u16 = 446;\n    pub const DATA_BIND_CONTEXT: u16 = 447;\n",
)
replace_once(
    "src/objects/core.rs",
    "    pub const DATA_BIND_FLAGS: u16 = 587;\n    pub const DATA_BIND_CONVERTER_ID: u16 = 660;\n",
    "    pub const DATA_BIND_FLAGS: u16 = 587;\n    pub const DATA_BIND_CONTEXT_SOURCE_PATH_IDS: u16 = 588;\n    pub const DATA_BIND_CONVERTER_ID: u16 = 660;\n",
)

replace_once(
    "src/objects/data_binding.rs",
    "impl RiveObject for DataBind {\n    fn type_key(&self) -> u16 {\n        type_keys::DATA_BIND\n    }\n\n    fn properties(&self) -> Vec<Property> {\n        let mut props = vec![\n            Property {\n                key: property_keys::DATA_BIND_PROPERTY_KEY,\n                value: PropertyValue::UInt(self.property_key),\n            },\n            Property {\n                key: property_keys::DATA_BIND_FLAGS,\n                value: PropertyValue::UInt(self.flags),\n            },\n        ];\n        if self.converter_id != u32::MAX as u64 {\n            props.push(Property {\n                key: property_keys::DATA_BIND_CONVERTER_ID,\n                value: PropertyValue::UInt(self.converter_id),\n            });\n        }\n        props\n    }\n}\n",
    "impl RiveObject for DataBind {\n    fn type_key(&self) -> u16 {\n        type_keys::DATA_BIND\n    }\n\n    fn properties(&self) -> Vec<Property> {\n        let mut props = vec![\n            Property {\n                key: property_keys::DATA_BIND_PROPERTY_KEY,\n                value: PropertyValue::UInt(self.property_key),\n            },\n            Property {\n                key: property_keys::DATA_BIND_FLAGS,\n                value: PropertyValue::UInt(self.flags),\n            },\n        ];\n        if self.converter_id != u32::MAX as u64 {\n            props.push(Property {\n                key: property_keys::DATA_BIND_CONVERTER_ID,\n                value: PropertyValue::UInt(self.converter_id),\n            });\n        }\n        props\n    }\n}\n\npub struct DataBindContext {\n    pub property_key: u64,\n    pub flags: u64,\n    pub source_path_ids: Vec<u8>,\n}\n\nimpl DataBindContext {\n    pub fn new(property_key: u64, flags: u64, source_path_ids: Vec<u8>) -> Self {\n        Self {\n            property_key,\n            flags,\n            source_path_ids,\n        }\n    }\n}\n\nimpl RiveObject for DataBindContext {\n    fn type_key(&self) -> u16 {\n        type_keys::DATA_BIND_CONTEXT\n    }\n\n    fn properties(&self) -> Vec<Property> {\n        vec![\n            Property {\n                key: property_keys::DATA_BIND_PROPERTY_KEY,\n                value: PropertyValue::UInt(self.property_key),\n            },\n            Property {\n                key: property_keys::DATA_BIND_FLAGS,\n                value: PropertyValue::UInt(self.flags),\n            },\n            Property {\n                key: property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS,\n                value: PropertyValue::Bytes(self.source_path_ids.clone()),\n            },\n        ]\n    }\n}\n",
)

replace_once(
    "src/builder/spec.rs",
    "#[derive(Debug, Deserialize, JsonSchema)]\n#[serde(tag = \"type\", rename_all = \"snake_case\")]\npub enum InputSpec {\n    Number { name: String, value: f32 },\n    Bool { name: String, value: bool },\n    Trigger { name: String },\n}\n",
    "#[derive(Debug, Deserialize, JsonSchema)]\npub struct ViewModelInputBindingSpec {\n    pub view_model: String,\n    pub property: String,\n}\n\n#[derive(Debug, Deserialize, JsonSchema)]\n#[serde(tag = \"type\", rename_all = \"snake_case\")]\npub enum InputSpec {\n    Number { name: String, value: f32 },\n    Bool {\n        name: String,\n        value: bool,\n        #[serde(default)]\n        view_model_binding: Option<ViewModelInputBindingSpec>,\n    },\n    Trigger { name: String },\n}\n",
)

replace_once(
    "src/builder/state_machines.rs",
    "use crate::objects::core::RiveObject;\n",
    "use crate::objects::core::{RiveObject, property_keys};\nuse crate::objects::data_binding::DataBindContext;\n",
)
replace_once(
    "src/builder/state_machines.rs",
    "                    InputSpec::Bool { name, value } => {\n                        objects.push(Box::new(StateMachineBool {\n                            name: name.clone(),\n                            value: if *value { 1 } else { 0 },\n                        }));\n                        input_name_to_index.insert(name.clone(), input_index);\n                    }\n",
    "                    InputSpec::Bool {\n                        name,\n                        value,\n                        view_model_binding,\n                    } => {\n                        objects.push(Box::new(StateMachineBool {\n                            name: name.clone(),\n                            value: if *value { 1 } else { 0 },\n                        }));\n                        if let Some(binding) = view_model_binding {\n                            let view_model_global = *object_name_to_index\n                                .get(&binding.view_model)\n                                .ok_or_else(|| {\n                                    format!(\n                                        \"unknown view model referenced by bool input '{}': '{}'\",\n                                        name, binding.view_model\n                                    )\n                                })?;\n                            let property_global = *object_name_to_index\n                                .get(&binding.property)\n                                .ok_or_else(|| {\n                                    format!(\n                                        \"unknown view-model property referenced by bool input '{}': '{}'\",\n                                        name, binding.property\n                                    )\n                                })?;\n                            let view_model_id = view_model_global\n                                .checked_sub(artboard_start)\n                                .ok_or_else(|| {\n                                    format!(\n                                        \"view model '{}' precedes current artboard\",\n                                        binding.view_model\n                                    )\n                                })? as u64;\n                            let property_id = property_global\n                                .checked_sub(artboard_start)\n                                .ok_or_else(|| {\n                                    format!(\n                                        \"view-model property '{}' precedes current artboard\",\n                                        binding.property\n                                    )\n                                })? as u64;\n                            objects.push(Box::new(DataBindContext::new(\n                                property_keys::STATE_MACHINE_BOOL_VALUE as u64,\n                                0,\n                                encode_id_path(&[view_model_id, property_id]),\n                            )));\n                        }\n                        input_name_to_index.insert(name.clone(), input_index);\n                    }\n",
)
replace_once(
    "src/builder/state_machines.rs",
    "/// Builds all state machine objects for an artboard.\n",
    "fn encode_id_path(ids: &[u64]) -> Vec<u8> {\n    let mut output = Vec::new();\n    for &id in ids {\n        let mut value = id;\n        loop {\n            let mut byte = (value & 0x7f) as u8;\n            value >>= 7;\n            if value != 0 {\n                byte |= 0x80;\n            }\n            output.push(byte);\n            if value == 0 {\n                break;\n            }\n        }\n    }\n    output\n}\n\n/// Builds all state machine objects for an artboard.\n",
)

print("issue #179 builder patch applied")
