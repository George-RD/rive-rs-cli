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

// ViewModel instance types (Unit 5)

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
