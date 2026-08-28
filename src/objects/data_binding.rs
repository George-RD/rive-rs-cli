use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct ViewModel {
    pub name: String,
    pub parent_id: u64,
}

impl ViewModel {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self { name, parent_id }
    }
}

impl RiveObject for ViewModel {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ViewModelProperty {
    pub name: String,
    pub parent_id: u64,
    pub property_type_value: u64,
}

impl ViewModelProperty {
    pub fn new(name: String, parent_id: u64, property_type_value: u64) -> Self {
        Self {
            name,
            parent_id,
            property_type_value,
        }
    }
}

impl RiveObject for ViewModelProperty {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.property_type_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_PROPERTY_TYPE_VALUE,
                value: PropertyValue::UInt(self.property_type_value),
            });
        }
        props
    }
}

pub struct DataBind {
    pub property_key: u64,
    pub flags: u64,
    pub converter_id: u64,
}

impl DataBind {
    pub fn new(property_key: u64, flags: u64) -> Self {
        Self {
            property_key,
            flags,
            converter_id: u32::MAX as u64,
        }
    }
}

impl RiveObject for DataBind {
    fn type_key(&self) -> u16 {
        type_keys::DATA_BIND
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::DATA_BIND_PROPERTY_KEY,
                value: PropertyValue::UInt(self.property_key),
            },
            Property {
                key: property_keys::DATA_BIND_FLAGS,
                value: PropertyValue::UInt(self.flags),
            },
        ];
        if self.converter_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::DATA_BIND_CONVERTER_ID,
                value: PropertyValue::UInt(self.converter_id),
            });
        }
        props
    }
}

pub struct DataBindContext {
    pub property_key: u64,
    pub flags: u64,
    pub source_path_ids: Vec<u8>,
}

impl DataBindContext {
    pub fn new(property_key: u64, flags: u64, source_path_ids: Vec<u8>) -> Self {
        Self {
            property_key,
            flags,
            source_path_ids,
        }
    }
}

impl RiveObject for DataBindContext {
    fn type_key(&self) -> u16 {
        type_keys::DATA_BIND_CONTEXT
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::DATA_BIND_PROPERTY_KEY,
                value: PropertyValue::UInt(self.property_key),
            },
            Property {
                key: property_keys::DATA_BIND_FLAGS,
                value: PropertyValue::UInt(self.flags),
            },
            Property {
                key: property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS,
                value: PropertyValue::Bytes(self.source_path_ids.clone()),
            },
        ]
    }
}

pub struct ViewModelInstance {
    pub view_model_id: u64,
}

impl RiveObject for ViewModelInstance {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VIEW_MODEL_ID,
                value: PropertyValue::UInt(self.view_model_id),
            });
        }
        props
    }
}

pub struct ViewModelInstanceValue {
    pub view_model_property_id: u64,
}

impl RiveObject for ViewModelInstanceValue {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_VALUE
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        props
    }
}

pub struct ViewModelInstanceColor {
    pub view_model_property_id: u64,
    pub property_value: u32,
}

impl RiveObject for ViewModelInstanceColor {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_COLOR
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        props.push(Property {
            key: property_keys::VIEW_MODEL_INSTANCE_COLOR_PROPERTY_VALUE,
            value: PropertyValue::Color(self.property_value),
        });
        props
    }
}

pub struct ViewModelInstanceString {
    pub view_model_property_id: u64,
    pub property_value: String,
}

impl RiveObject for ViewModelInstanceString {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_STRING
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        props.push(Property {
            key: property_keys::VIEW_MODEL_INSTANCE_STRING_PROPERTY_VALUE,
            value: PropertyValue::String(self.property_value.clone()),
        });
        props
    }
}

pub struct ViewModelInstanceNumber {
    pub view_model_property_id: u64,
    pub property_value: f32,
}

impl RiveObject for ViewModelInstanceNumber {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_NUMBER
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        props.push(Property {
            key: property_keys::VIEW_MODEL_INSTANCE_NUMBER_PROPERTY_VALUE,
            value: PropertyValue::Float(self.property_value),
        });
        props
    }
}

pub struct ViewModelInstanceBoolean {
    pub view_model_property_id: u64,
    pub property_value: bool,
}

impl RiveObject for ViewModelInstanceBoolean {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_BOOLEAN
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        props.push(Property {
            key: property_keys::VIEW_MODEL_INSTANCE_BOOLEAN_PROPERTY_VALUE,
            value: PropertyValue::Bool(self.property_value),
        });
        props
    }
}

pub struct ViewModelInstanceEnum {
    pub view_model_property_id: u64,
    pub property_value: u64,
}

impl RiveObject for ViewModelInstanceEnum {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_ENUM
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_ENUM_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct ViewModelInstanceList;

impl RiveObject for ViewModelInstanceList {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_LIST
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct ViewModelInstanceListItem {
    pub view_model_id: u64,
    pub view_model_instance_id: u64,
}

impl RiveObject for ViewModelInstanceListItem {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_LIST_ITEM
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_LIST_ITEM_VIEW_MODEL_ID,
                value: PropertyValue::UInt(self.view_model_id),
            });
        }
        if self.view_model_instance_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_LIST_ITEM_VIEW_MODEL_INSTANCE_ID,
                value: PropertyValue::UInt(self.view_model_instance_id),
            });
        }
        props
    }
}

pub struct ViewModelInstanceViewModel {
    pub view_model_property_id: u64,
    pub property_value: u64,
}

impl RiveObject for ViewModelInstanceViewModel {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_VIEW_MODEL
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VIEW_MODEL_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct ViewModelPropertyNumber {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ViewModelPropertyNumber {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_NUMBER
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ViewModelPropertyBoolean {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ViewModelPropertyBoolean {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_BOOLEAN
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ViewModelPropertyString {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ViewModelPropertyString {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_STRING
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ViewModelPropertyColor {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ViewModelPropertyColor {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_COLOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ViewModelPropertyList {
    pub name: String,
    pub parent_id: u64,
    pub view_model_reference_id: u64,
}

impl RiveObject for ViewModelPropertyList {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_LIST
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.view_model_reference_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_REFERENCE_ID,
                value: PropertyValue::UInt(self.view_model_reference_id),
            });
        }
        props
    }
}

pub struct ViewModelPropertyViewModel {
    pub name: String,
    pub parent_id: u64,
    pub view_model_reference_id: u64,
}

impl RiveObject for ViewModelPropertyViewModel {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_COMPONENT
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.view_model_reference_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_REFERENCE_ID,
                value: PropertyValue::UInt(self.view_model_reference_id),
            });
        }
        props
    }
}

pub struct ViewModelPropertyEnum {
    pub name: String,
    pub parent_id: u64,
    pub enum_id: u64,
}

impl RiveObject for ViewModelPropertyEnum {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_ENUM
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.enum_id != 0 {
            props.push(Property {
                key: property_keys::DATA_ENUM_ENUM_ID,
                value: PropertyValue::UInt(self.enum_id),
            });
        }
        props
    }
}

pub struct ViewModelPropertyEnumCustom {
    pub name: String,
    pub parent_id: u64,
    pub enum_id: u64,
}

impl RiveObject for ViewModelPropertyEnumCustom {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_ENUM_CUSTOM
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.enum_id != 0 {
            props.push(Property {
                key: property_keys::DATA_ENUM_CUSTOM_ENUM_ID,
                value: PropertyValue::UInt(self.enum_id),
            });
        }
        props
    }
}

pub struct ViewModelPropertyEnumSystem {
    pub name: String,
    pub parent_id: u64,
    pub enum_id: u64,
}

impl RiveObject for ViewModelPropertyEnumSystem {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_ENUM_SYSTEM
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.enum_id != 0 {
            props.push(Property {
                key: property_keys::DATA_ENUM_ENUM_ID,
                value: PropertyValue::UInt(self.enum_id),
            });
        }
        props
    }
}

pub struct ViewModelPropertyTrigger {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ViewModelPropertyTrigger {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_TRIGGER
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ViewModelPropertyAssetImage {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ViewModelPropertyAssetImage {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_ASSET_IMAGE
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ViewModelPropertyArtboard {
    pub name: String,
    pub parent_id: u64,
    pub artboard_id: u64,
}

impl RiveObject for ViewModelPropertyArtboard {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_ARTBOARD
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.artboard_id != 0 {
            props.push(Property {
                key: property_keys::VM_PROPERTY_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            });
        }
        props
    }
}

pub struct ViewModelPropertySymbol {
    pub name: String,
    pub parent_id: u64,
    pub symbol_type_value: u64,
    pub artboard_id: u64,
}

impl RiveObject for ViewModelPropertySymbol {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_SYMBOL
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.symbol_type_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_PROPERTY_TYPE_VALUE_SYMBOL,
                value: PropertyValue::UInt(self.symbol_type_value),
            });
        }
        if self.artboard_id != 0 {
            props.push(Property {
                key: property_keys::VM_PROPERTY_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            });
        }
        props
    }
}

pub struct ViewModelPropertySymbolListIndex {
    pub name: String,
    pub parent_id: u64,
    pub symbol_type_value: u64,
    pub artboard_id: u64,
    pub list_source: u64,
}

impl RiveObject for ViewModelPropertySymbolListIndex {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_SYMBOL_LIST_INDEX
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.symbol_type_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_PROPERTY_TYPE_VALUE_SYMBOL,
                value: PropertyValue::UInt(self.symbol_type_value),
            });
        }
        if self.artboard_id != 0 {
            props.push(Property {
                key: property_keys::VM_PROPERTY_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            });
        }
        if self.list_source != 0 {
            props.push(Property {
                key: property_keys::LIST_SOURCE,
                value: PropertyValue::UInt(self.list_source),
            });
        }
        props
    }
}

pub struct ViewModelInstanceTrigger {
    pub view_model_property_id: u64,
    pub property_value: u64,
}

impl RiveObject for ViewModelInstanceTrigger {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_TRIGGER
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_TRIGGER_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct ViewModelInstanceSymbol {
    pub view_model_property_id: u64,
    pub property_value: u64,
}

impl RiveObject for ViewModelInstanceSymbol {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_SYMBOL
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_SYMBOL_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct ViewModelInstanceSymbolListIndex {
    pub view_model_property_id: u64,
    pub property_value: u64,
}

impl RiveObject for ViewModelInstanceSymbolListIndex {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_SYMBOL_LIST_INDEX
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_SYMBOL_LIST_INDEX_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct ViewModelInstanceAssetImage {
    pub view_model_property_id: u64,
    pub property_value: u64,
}

impl RiveObject for ViewModelInstanceAssetImage {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_ASSET_IMAGE
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_ASSET_IMAGE_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct ViewModelInstanceArtboard {
    pub view_model_property_id: u64,
    pub property_value: u64,
    pub artboard_id: u64,
}

impl RiveObject for ViewModelInstanceArtboard {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_INSTANCE_ARTBOARD
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_ARTBOARD_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        if self.artboard_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_ARTBOARD_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            });
        }
        props
    }
}

pub struct DataEnum {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for DataEnum {
    fn type_key(&self) -> u16 {
        type_keys::DATA_ENUM
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct DataEnumCustom {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for DataEnumCustom {
    fn type_key(&self) -> u16 {
        type_keys::DATA_ENUM_CUSTOM
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::DATA_ENUM_CUSTOM_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct DataEnumValue {
    pub key: String,
    pub value: String,
}

impl RiveObject for DataEnumValue {
    fn type_key(&self) -> u16 {
        type_keys::DATA_ENUM_VALUE
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::DATA_ENUM_VALUE_KEY,
                value: PropertyValue::String(self.key.clone()),
            },
            Property {
                key: property_keys::DATA_ENUM_VALUE_VALUE,
                value: PropertyValue::String(self.value.clone()),
            },
        ]
    }
}

pub struct DataEnumSystem {
    pub name: String,
    pub parent_id: u64,
    pub enum_type: u64,
}

impl RiveObject for DataEnumSystem {
    fn type_key(&self) -> u16 {
        type_keys::DATA_ENUM_SYSTEM
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.enum_type != 0 {
            props.push(Property {
                key: property_keys::DATA_ENUM_ENUM_TYPE,
                value: PropertyValue::UInt(self.enum_type),
            });
        }
        props
    }
}

pub struct BindablePropertyString {
    pub property_value: String,
}

impl RiveObject for BindablePropertyString {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_STRING
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::BINDABLE_PROPERTY_STRING_VALUE,
            value: PropertyValue::String(self.property_value.clone()),
        }]
    }
}

pub struct BindablePropertyBoolean {
    pub property_value: u64,
}

impl RiveObject for BindablePropertyBoolean {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_BOOLEAN
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::BINDABLE_PROPERTY_BOOLEAN_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct BindablePropertyNumber {
    pub property_value: f32,
}

impl RiveObject for BindablePropertyNumber {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_NUMBER
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.property_value != 0.0 {
            props.push(Property {
                key: property_keys::BINDABLE_PROPERTY_NUMBER_VALUE,
                value: PropertyValue::Float(self.property_value),
            });
        }
        props
    }
}

pub struct BindablePropertyEnum {
    pub property_value: u64,
}

impl RiveObject for BindablePropertyEnum {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_ENUM
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::BINDABLE_PROPERTY_ENUM_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct BindablePropertyColor {
    pub property_value: u32,
}

impl RiveObject for BindablePropertyColor {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_COLOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::BINDABLE_PROPERTY_COLOR_VALUE,
            value: PropertyValue::Color(self.property_value),
        }]
    }
}

pub struct BindablePropertyTrigger {
    pub property_value: u64,
}

impl RiveObject for BindablePropertyTrigger {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_TRIGGER
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::BINDABLE_PROPERTY_TRIGGER_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct BindablePropertyInteger {
    pub property_value: u64,
}

impl RiveObject for BindablePropertyInteger {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_INTEGER
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::BINDABLE_PROPERTY_INTEGER_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct BindablePropertyList {
    pub property_value: u64,
}

impl RiveObject for BindablePropertyList {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_LIST
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::BINDABLE_PROPERTY_LIST_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct BindablePropertyId {
    pub property_value: u32,
}

impl RiveObject for BindablePropertyId {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_ID
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::BINDABLE_PROPERTY_ID_VALUE,
            value: PropertyValue::Color(self.property_value),
        }]
    }
}

pub struct BindablePropertyArtboard {
    pub property_value: u64,
}

impl RiveObject for BindablePropertyArtboard {
    fn type_key(&self) -> u16 {
        type_keys::BINDABLE_PROPERTY_ARTBOARD
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::BINDABLE_PROPERTY_ARTBOARD_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct DataBindPath {
    pub property_key: u64,
    pub flags: u64,
    pub converter_id: u64,
}

impl DataBindPath {
    pub fn new(property_key: u64, flags: u64) -> Self {
        Self {
            property_key,
            flags,
            converter_id: u32::MAX as u64,
        }
    }
}

impl RiveObject for DataBindPath {
    fn type_key(&self) -> u16 {
        type_keys::DATA_BIND_PATH
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::DATA_BIND_PROPERTY_KEY,
                value: PropertyValue::UInt(self.property_key),
            },
            Property {
                key: property_keys::DATA_BIND_FLAGS,
                value: PropertyValue::UInt(self.flags),
            },
        ];
        if self.converter_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::DATA_BIND_CONVERTER_ID,
                value: PropertyValue::UInt(self.converter_id),
            });
        }
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::core::{PropertyValue, property_keys, type_keys};

    #[test]
    fn test_view_model_type_key() {
        let vm = ViewModel::new("vm1".to_string(), 0);
        assert_eq!(vm.type_key(), type_keys::VIEW_MODEL);
    }

    #[test]
    fn test_view_model_properties() {
        let vm = ViewModel::new("vm1".to_string(), 1);
        let props = vm.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].value, PropertyValue::String("vm1".to_string()));
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_view_model_property_type_key() {
        let vmp = ViewModelProperty::new("prop1".to_string(), 0, 1);
        assert_eq!(vmp.type_key(), type_keys::VIEW_MODEL_PROPERTY);
    }

    #[test]
    fn test_view_model_property_default_type() {
        let vmp = ViewModelProperty::new("prop1".to_string(), 0, 0);
        let props = vmp.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_view_model_property_with_type() {
        let vmp = ViewModelProperty::new("prop1".to_string(), 0, 3);
        let props = vmp.properties();
        assert_eq!(props.len(), 3);
        let type_prop = props
            .iter()
            .find(|p| p.key == property_keys::VIEW_MODEL_PROPERTY_TYPE_VALUE)
            .unwrap();
        assert_eq!(type_prop.value, PropertyValue::UInt(3));
    }

    #[test]
    fn test_data_bind_type_key() {
        let db = DataBind::new(42, 1);
        assert_eq!(db.type_key(), type_keys::DATA_BIND);
    }

    #[test]
    fn test_data_bind_default_properties() {
        let db = DataBind::new(42, 1);
        let props = db.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::DATA_BIND_PROPERTY_KEY);
        assert_eq!(props[0].value, PropertyValue::UInt(42));
        assert_eq!(props[1].key, property_keys::DATA_BIND_FLAGS);
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_data_bind_with_converter() {
        let mut db = DataBind::new(42, 1);
        db.converter_id = 5;
        let props = db.properties();
        assert_eq!(props.len(), 3);
        let conv_prop = props
            .iter()
            .find(|p| p.key == property_keys::DATA_BIND_CONVERTER_ID)
            .unwrap();
        assert_eq!(conv_prop.value, PropertyValue::UInt(5));
    }

    #[test]
    fn test_data_bind_no_name_or_parent() {
        let db = DataBind::new(10, 0);
        let props = db.properties();
        assert!(!props.iter().any(|p| p.key == property_keys::COMPONENT_NAME));
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID)
        );
    }
}
