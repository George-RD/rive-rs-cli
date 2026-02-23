use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct Bone {
    pub name: String,
    pub parent_id: u64,
    pub length: f32,
}

impl Bone {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            length: 0.0,
        }
    }
}

impl RiveObject for Bone {
    fn type_key(&self) -> u16 {
        type_keys::BONE
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
        if self.length != 0.0 {
            props.push(Property {
                key: property_keys::BONE_LENGTH,
                value: PropertyValue::Float(self.length),
            });
        }
        props
    }
}

pub struct RootBone {
    pub name: String,
    pub parent_id: u64,
    pub length: f32,
    pub x: f32,
    pub y: f32,
}

impl RootBone {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            length: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl RiveObject for RootBone {
    fn type_key(&self) -> u16 {
        type_keys::ROOT_BONE
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
        if self.length != 0.0 {
            props.push(Property {
                key: property_keys::BONE_LENGTH,
                value: PropertyValue::Float(self.length),
            });
        }
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::ROOT_BONE_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::ROOT_BONE_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        props
    }
}

pub struct Skin {
    pub name: String,
    pub parent_id: u64,
    pub xx: f32,
    pub yx: f32,
    pub xy: f32,
    pub yy: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Skin {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            xx: 1.0,
            yx: 0.0,
            xy: 0.0,
            yy: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}

impl RiveObject for Skin {
    fn type_key(&self) -> u16 {
        type_keys::SKIN
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
        if self.xx != 1.0 {
            props.push(Property {
                key: property_keys::SKIN_XX,
                value: PropertyValue::Float(self.xx),
            });
        }
        if self.yx != 0.0 {
            props.push(Property {
                key: property_keys::SKIN_YX,
                value: PropertyValue::Float(self.yx),
            });
        }
        if self.xy != 0.0 {
            props.push(Property {
                key: property_keys::SKIN_XY,
                value: PropertyValue::Float(self.xy),
            });
        }
        if self.yy != 1.0 {
            props.push(Property {
                key: property_keys::SKIN_YY,
                value: PropertyValue::Float(self.yy),
            });
        }
        if self.tx != 0.0 {
            props.push(Property {
                key: property_keys::SKIN_TX,
                value: PropertyValue::Float(self.tx),
            });
        }
        if self.ty != 0.0 {
            props.push(Property {
                key: property_keys::SKIN_TY,
                value: PropertyValue::Float(self.ty),
            });
        }
        props
    }
}

pub struct Tendon {
    pub name: String,
    pub parent_id: u64,
    pub bone_id: u64,
    pub xx: f32,
    pub yx: f32,
    pub xy: f32,
    pub yy: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Tendon {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            bone_id: u32::MAX as u64,
            xx: 1.0,
            yx: 0.0,
            xy: 0.0,
            yy: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}

impl RiveObject for Tendon {
    fn type_key(&self) -> u16 {
        type_keys::TENDON
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
        if self.bone_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TENDON_BONE_ID,
                value: PropertyValue::UInt(self.bone_id),
            });
        }
        if self.xx != 1.0 {
            props.push(Property {
                key: property_keys::TENDON_XX,
                value: PropertyValue::Float(self.xx),
            });
        }
        if self.yx != 0.0 {
            props.push(Property {
                key: property_keys::TENDON_YX,
                value: PropertyValue::Float(self.yx),
            });
        }
        if self.xy != 0.0 {
            props.push(Property {
                key: property_keys::TENDON_XY,
                value: PropertyValue::Float(self.xy),
            });
        }
        if self.yy != 1.0 {
            props.push(Property {
                key: property_keys::TENDON_YY,
                value: PropertyValue::Float(self.yy),
            });
        }
        if self.tx != 0.0 {
            props.push(Property {
                key: property_keys::TENDON_TX,
                value: PropertyValue::Float(self.tx),
            });
        }
        if self.ty != 0.0 {
            props.push(Property {
                key: property_keys::TENDON_TY,
                value: PropertyValue::Float(self.ty),
            });
        }
        props
    }
}

pub struct Weight {
    pub name: String,
    pub parent_id: u64,
    pub values: u64,
    pub indices: u64,
}

impl Weight {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            values: 255,
            indices: 1,
        }
    }
}

impl RiveObject for Weight {
    fn type_key(&self) -> u16 {
        type_keys::WEIGHT
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
        if self.values != 255 {
            props.push(Property {
                key: property_keys::WEIGHT_VALUES,
                value: PropertyValue::UInt(self.values),
            });
        }
        if self.indices != 1 {
            props.push(Property {
                key: property_keys::WEIGHT_INDICES,
                value: PropertyValue::UInt(self.indices),
            });
        }
        props
    }
}

pub struct CubicWeight {
    pub name: String,
    pub parent_id: u64,
    pub in_values: u64,
    pub in_indices: u64,
    pub out_values: u64,
    pub out_indices: u64,
}

impl CubicWeight {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            in_values: 255,
            in_indices: 1,
            out_values: 255,
            out_indices: 1,
        }
    }
}

impl RiveObject for CubicWeight {
    fn type_key(&self) -> u16 {
        type_keys::CUBIC_WEIGHT
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
        if self.in_values != 255 {
            props.push(Property {
                key: property_keys::CUBIC_WEIGHT_IN_VALUES,
                value: PropertyValue::UInt(self.in_values),
            });
        }
        if self.in_indices != 1 {
            props.push(Property {
                key: property_keys::CUBIC_WEIGHT_IN_INDICES,
                value: PropertyValue::UInt(self.in_indices),
            });
        }
        if self.out_values != 255 {
            props.push(Property {
                key: property_keys::CUBIC_WEIGHT_OUT_VALUES,
                value: PropertyValue::UInt(self.out_values),
            });
        }
        if self.out_indices != 1 {
            props.push(Property {
                key: property_keys::CUBIC_WEIGHT_OUT_INDICES,
                value: PropertyValue::UInt(self.out_indices),
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
    fn test_bone_type_key() {
        let bone = Bone::new("bone1".to_string(), 0);
        assert_eq!(bone.type_key(), type_keys::BONE);
    }

    #[test]
    fn test_bone_default_properties() {
        let bone = Bone::new("bone1".to_string(), 1);
        let props = bone.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].value, PropertyValue::String("bone1".to_string()));
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_bone_with_length() {
        let mut bone = Bone::new("bone1".to_string(), 0);
        bone.length = 50.0;
        let props = bone.properties();
        assert_eq!(props.len(), 3);
        let length_prop = props
            .iter()
            .find(|p| p.key == property_keys::BONE_LENGTH)
            .unwrap();
        assert_eq!(length_prop.value, PropertyValue::Float(50.0));
    }

    #[test]
    fn test_root_bone_type_key() {
        let root = RootBone::new("root".to_string(), 0);
        assert_eq!(root.type_key(), type_keys::ROOT_BONE);
    }

    #[test]
    fn test_root_bone_default_properties() {
        let root = RootBone::new("root".to_string(), 0);
        let props = root.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_root_bone_with_position() {
        let mut root = RootBone::new("root".to_string(), 0);
        root.x = 100.0;
        root.y = 200.0;
        root.length = 30.0;
        let props = root.properties();
        assert_eq!(props.len(), 5);
        let x_prop = props
            .iter()
            .find(|p| p.key == property_keys::ROOT_BONE_X)
            .unwrap();
        assert_eq!(x_prop.value, PropertyValue::Float(100.0));
        let y_prop = props
            .iter()
            .find(|p| p.key == property_keys::ROOT_BONE_Y)
            .unwrap();
        assert_eq!(y_prop.value, PropertyValue::Float(200.0));
    }

    #[test]
    fn test_skin_type_key() {
        let skin = Skin::new("skin1".to_string(), 0);
        assert_eq!(skin.type_key(), type_keys::SKIN);
    }

    #[test]
    fn test_skin_default_identity_matrix() {
        let skin = Skin::new("skin1".to_string(), 0);
        let props = skin.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_skin_custom_transform() {
        let mut skin = Skin::new("skin1".to_string(), 0);
        skin.xx = 2.0;
        skin.ty = 10.0;
        let props = skin.properties();
        assert_eq!(props.len(), 4);
        let xx_prop = props
            .iter()
            .find(|p| p.key == property_keys::SKIN_XX)
            .unwrap();
        assert_eq!(xx_prop.value, PropertyValue::Float(2.0));
        let ty_prop = props
            .iter()
            .find(|p| p.key == property_keys::SKIN_TY)
            .unwrap();
        assert_eq!(ty_prop.value, PropertyValue::Float(10.0));
    }

    #[test]
    fn test_tendon_type_key() {
        let tendon = Tendon::new("tendon1".to_string(), 0);
        assert_eq!(tendon.type_key(), type_keys::TENDON);
    }

    #[test]
    fn test_tendon_default_bone_id() {
        let tendon = Tendon::new("tendon1".to_string(), 0);
        let props = tendon.properties();
        assert_eq!(props.len(), 2);
        assert!(!props.iter().any(|p| p.key == property_keys::TENDON_BONE_ID));
    }

    #[test]
    fn test_tendon_with_bone_id() {
        let mut tendon = Tendon::new("tendon1".to_string(), 0);
        tendon.bone_id = 3;
        let props = tendon.properties();
        let bone_id_prop = props
            .iter()
            .find(|p| p.key == property_keys::TENDON_BONE_ID)
            .unwrap();
        assert_eq!(bone_id_prop.value, PropertyValue::UInt(3));
    }

    #[test]
    fn test_weight_type_key() {
        let weight = Weight::new("weight1".to_string(), 0);
        assert_eq!(weight.type_key(), type_keys::WEIGHT);
    }

    #[test]
    fn test_weight_default_properties() {
        let weight = Weight::new("weight1".to_string(), 0);
        let props = weight.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_weight_custom_values() {
        let mut weight = Weight::new("weight1".to_string(), 0);
        weight.values = 128;
        weight.indices = 2;
        let props = weight.properties();
        assert_eq!(props.len(), 4);
        let values_prop = props
            .iter()
            .find(|p| p.key == property_keys::WEIGHT_VALUES)
            .unwrap();
        assert_eq!(values_prop.value, PropertyValue::UInt(128));
        let indices_prop = props
            .iter()
            .find(|p| p.key == property_keys::WEIGHT_INDICES)
            .unwrap();
        assert_eq!(indices_prop.value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_cubic_weight_type_key() {
        let cw = CubicWeight::new("cw1".to_string(), 0);
        assert_eq!(cw.type_key(), type_keys::CUBIC_WEIGHT);
    }

    #[test]
    fn test_cubic_weight_default_properties() {
        let cw = CubicWeight::new("cw1".to_string(), 0);
        let props = cw.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_cubic_weight_custom_values() {
        let mut cw = CubicWeight::new("cw1".to_string(), 0);
        cw.in_values = 64;
        cw.out_indices = 3;
        let props = cw.properties();
        assert_eq!(props.len(), 4);
        let in_val_prop = props
            .iter()
            .find(|p| p.key == property_keys::CUBIC_WEIGHT_IN_VALUES)
            .unwrap();
        assert_eq!(in_val_prop.value, PropertyValue::UInt(64));
        let out_idx_prop = props
            .iter()
            .find(|p| p.key == property_keys::CUBIC_WEIGHT_OUT_INDICES)
            .unwrap();
        assert_eq!(out_idx_prop.value, PropertyValue::UInt(3));
    }
}
