use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct IKConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,
    pub target_id: u64,
    pub invert_direction: bool,
    pub parent_bone_count: u64,
}

impl IKConstraint {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            strength: 1.0,
            target_id: u32::MAX as u64,
            invert_direction: false,
            parent_bone_count: 0,
        }
    }
}

impl RiveObject for IKConstraint {
    fn type_key(&self) -> u16 {
        type_keys::IK_CONSTRAINT
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
        if self.strength != 1.0 {
            props.push(Property {
                key: property_keys::CONSTRAINT_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.target_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TARGETED_CONSTRAINT_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        if self.invert_direction {
            props.push(Property {
                key: property_keys::IK_CONSTRAINT_INVERT_DIRECTION,
                value: PropertyValue::UInt(1),
            });
        }
        if self.parent_bone_count != 0 {
            props.push(Property {
                key: property_keys::IK_CONSTRAINT_PARENT_BONE_COUNT,
                value: PropertyValue::UInt(self.parent_bone_count),
            });
        }
        props
    }
}

pub struct DistanceConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,
    pub target_id: u64,
    pub distance: f32,
    pub mode_value: u64,
}

impl DistanceConstraint {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            strength: 1.0,
            target_id: u32::MAX as u64,
            distance: 100.0,
            mode_value: 0,
        }
    }
}

impl RiveObject for DistanceConstraint {
    fn type_key(&self) -> u16 {
        type_keys::DISTANCE_CONSTRAINT
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
        if self.strength != 1.0 {
            props.push(Property {
                key: property_keys::CONSTRAINT_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.target_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TARGETED_CONSTRAINT_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        if self.distance != 100.0 {
            props.push(Property {
                key: property_keys::DISTANCE_CONSTRAINT_DISTANCE,
                value: PropertyValue::Float(self.distance),
            });
        }
        if self.mode_value != 0 {
            props.push(Property {
                key: property_keys::DISTANCE_CONSTRAINT_MODE_VALUE,
                value: PropertyValue::UInt(self.mode_value),
            });
        }
        props
    }
}

pub struct TransformConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,
    pub target_id: u64,
    pub source_space_value: u64,
    pub dest_space_value: u64,
    pub origin_x: f32,
    pub origin_y: f32,
}

impl TransformConstraint {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            strength: 1.0,
            target_id: u32::MAX as u64,
            source_space_value: 0,
            dest_space_value: 0,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }
}

impl RiveObject for TransformConstraint {
    fn type_key(&self) -> u16 {
        type_keys::TRANSFORM_CONSTRAINT
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
        if self.strength != 1.0 {
            props.push(Property {
                key: property_keys::CONSTRAINT_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.target_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TARGETED_CONSTRAINT_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        if self.source_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_SOURCE_SPACE_VALUE,
                value: PropertyValue::UInt(self.source_space_value),
            });
        }
        if self.dest_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_DEST_SPACE_VALUE,
                value: PropertyValue::UInt(self.dest_space_value),
            });
        }
        if self.origin_x != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_CONSTRAINT_ORIGIN_X,
                value: PropertyValue::Float(self.origin_x),
            });
        }
        if self.origin_y != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_CONSTRAINT_ORIGIN_Y,
                value: PropertyValue::Float(self.origin_y),
            });
        }
        props
    }
}

pub struct TranslationConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,
    pub target_id: u64,
    pub source_space_value: u64,
    pub dest_space_value: u64,
    pub copy_factor: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub offset: bool,
    pub does_copy: bool,
    pub min: bool,
    pub max: bool,
    pub min_max_space_value: u64,
    pub copy_factor_y: f32,
    pub min_value_y: f32,
    pub max_value_y: f32,
    pub does_copy_y: bool,
    pub min_y: bool,
    pub max_y: bool,
}

impl TranslationConstraint {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            strength: 1.0,
            target_id: u32::MAX as u64,
            source_space_value: 0,
            dest_space_value: 0,
            copy_factor: 1.0,
            min_value: 0.0,
            max_value: 0.0,
            offset: false,
            does_copy: true,
            min: false,
            max: false,
            min_max_space_value: 0,
            copy_factor_y: 1.0,
            min_value_y: 0.0,
            max_value_y: 0.0,
            does_copy_y: true,
            min_y: false,
            max_y: false,
        }
    }
}

impl RiveObject for TranslationConstraint {
    fn type_key(&self) -> u16 {
        type_keys::TRANSLATION_CONSTRAINT
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
        if self.strength != 1.0 {
            props.push(Property {
                key: property_keys::CONSTRAINT_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.target_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TARGETED_CONSTRAINT_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        if self.source_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_SOURCE_SPACE_VALUE,
                value: PropertyValue::UInt(self.source_space_value),
            });
        }
        if self.dest_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_DEST_SPACE_VALUE,
                value: PropertyValue::UInt(self.dest_space_value),
            });
        }
        if self.copy_factor != 1.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_COPY_FACTOR,
                value: PropertyValue::Float(self.copy_factor),
            });
        }
        if self.min_value != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN_VALUE,
                value: PropertyValue::Float(self.min_value),
            });
        }
        if self.max_value != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX_VALUE,
                value: PropertyValue::Float(self.max_value),
            });
        }
        if self.offset {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_OFFSET,
                value: PropertyValue::UInt(1),
            });
        }
        if !self.does_copy {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY,
                value: PropertyValue::UInt(0),
            });
        }
        if self.min {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN,
                value: PropertyValue::UInt(1),
            });
        }
        if self.max {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX,
                value: PropertyValue::UInt(1),
            });
        }
        if self.min_max_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN_MAX_SPACE_VALUE,
                value: PropertyValue::UInt(self.min_max_space_value),
            });
        }
        if self.copy_factor_y != 1.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_COPY_FACTOR_Y,
                value: PropertyValue::Float(self.copy_factor_y),
            });
        }
        if self.min_value_y != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_VALUE_Y,
                value: PropertyValue::Float(self.min_value_y),
            });
        }
        if self.max_value_y != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_VALUE_Y,
                value: PropertyValue::Float(self.max_value_y),
            });
        }
        if !self.does_copy_y {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_DOES_COPY_Y,
                value: PropertyValue::UInt(0),
            });
        }
        if self.min_y {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_Y,
                value: PropertyValue::UInt(1),
            });
        }
        if self.max_y {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_Y,
                value: PropertyValue::UInt(1),
            });
        }
        props
    }
}

pub struct ScaleConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,
    pub target_id: u64,
    pub source_space_value: u64,
    pub dest_space_value: u64,
    pub copy_factor: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub offset: bool,
    pub does_copy: bool,
    pub min: bool,
    pub max: bool,
    pub min_max_space_value: u64,
    pub copy_factor_y: f32,
    pub min_value_y: f32,
    pub max_value_y: f32,
    pub does_copy_y: bool,
    pub min_y: bool,
    pub max_y: bool,
}

impl ScaleConstraint {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            strength: 1.0,
            target_id: u32::MAX as u64,
            source_space_value: 0,
            dest_space_value: 0,
            copy_factor: 1.0,
            min_value: 0.0,
            max_value: 0.0,
            offset: false,
            does_copy: true,
            min: false,
            max: false,
            min_max_space_value: 0,
            copy_factor_y: 1.0,
            min_value_y: 0.0,
            max_value_y: 0.0,
            does_copy_y: true,
            min_y: false,
            max_y: false,
        }
    }
}

impl RiveObject for ScaleConstraint {
    fn type_key(&self) -> u16 {
        type_keys::SCALE_CONSTRAINT
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
        if self.strength != 1.0 {
            props.push(Property {
                key: property_keys::CONSTRAINT_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.target_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TARGETED_CONSTRAINT_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        if self.source_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_SOURCE_SPACE_VALUE,
                value: PropertyValue::UInt(self.source_space_value),
            });
        }
        if self.dest_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_DEST_SPACE_VALUE,
                value: PropertyValue::UInt(self.dest_space_value),
            });
        }
        if self.copy_factor != 1.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_COPY_FACTOR,
                value: PropertyValue::Float(self.copy_factor),
            });
        }
        if self.min_value != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN_VALUE,
                value: PropertyValue::Float(self.min_value),
            });
        }
        if self.max_value != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX_VALUE,
                value: PropertyValue::Float(self.max_value),
            });
        }
        if self.offset {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_OFFSET,
                value: PropertyValue::UInt(1),
            });
        }
        if !self.does_copy {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY,
                value: PropertyValue::UInt(0),
            });
        }
        if self.min {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN,
                value: PropertyValue::UInt(1),
            });
        }
        if self.max {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX,
                value: PropertyValue::UInt(1),
            });
        }
        if self.min_max_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN_MAX_SPACE_VALUE,
                value: PropertyValue::UInt(self.min_max_space_value),
            });
        }
        if self.copy_factor_y != 1.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_COPY_FACTOR_Y,
                value: PropertyValue::Float(self.copy_factor_y),
            });
        }
        if self.min_value_y != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_VALUE_Y,
                value: PropertyValue::Float(self.min_value_y),
            });
        }
        if self.max_value_y != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_VALUE_Y,
                value: PropertyValue::Float(self.max_value_y),
            });
        }
        if !self.does_copy_y {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_DOES_COPY_Y,
                value: PropertyValue::UInt(0),
            });
        }
        if self.min_y {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_Y,
                value: PropertyValue::UInt(1),
            });
        }
        if self.max_y {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_Y,
                value: PropertyValue::UInt(1),
            });
        }
        props
    }
}

pub struct RotationConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,
    pub target_id: u64,
    pub source_space_value: u64,
    pub dest_space_value: u64,
    pub copy_factor: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub offset: bool,
    pub does_copy: bool,
    pub min: bool,
    pub max: bool,
    pub min_max_space_value: u64,
}

impl RotationConstraint {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            strength: 1.0,
            target_id: u32::MAX as u64,
            source_space_value: 0,
            dest_space_value: 0,
            copy_factor: 1.0,
            min_value: 0.0,
            max_value: 0.0,
            offset: false,
            does_copy: true,
            min: false,
            max: false,
            min_max_space_value: 0,
        }
    }
}

impl RiveObject for RotationConstraint {
    fn type_key(&self) -> u16 {
        type_keys::ROTATION_CONSTRAINT
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
        if self.strength != 1.0 {
            props.push(Property {
                key: property_keys::CONSTRAINT_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.target_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TARGETED_CONSTRAINT_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        if self.source_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_SOURCE_SPACE_VALUE,
                value: PropertyValue::UInt(self.source_space_value),
            });
        }
        if self.dest_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SPACE_DEST_SPACE_VALUE,
                value: PropertyValue::UInt(self.dest_space_value),
            });
        }
        if self.copy_factor != 1.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_COPY_FACTOR,
                value: PropertyValue::Float(self.copy_factor),
            });
        }
        if self.min_value != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN_VALUE,
                value: PropertyValue::Float(self.min_value),
            });
        }
        if self.max_value != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX_VALUE,
                value: PropertyValue::Float(self.max_value),
            });
        }
        if self.offset {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_OFFSET,
                value: PropertyValue::UInt(1),
            });
        }
        if !self.does_copy {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY,
                value: PropertyValue::UInt(0),
            });
        }
        if self.min {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN,
                value: PropertyValue::UInt(1),
            });
        }
        if self.max {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX,
                value: PropertyValue::UInt(1),
            });
        }
        if self.min_max_space_value != 0 {
            props.push(Property {
                key: property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN_MAX_SPACE_VALUE,
                value: PropertyValue::UInt(self.min_max_space_value),
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
    fn test_ik_constraint_type_key() {
        let ik = IKConstraint::new("ik1".to_string(), 0);
        assert_eq!(ik.type_key(), type_keys::IK_CONSTRAINT);
    }

    #[test]
    fn test_ik_constraint_default_properties() {
        let ik = IKConstraint::new("ik1".to_string(), 1);
        let props = ik.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].value, PropertyValue::String("ik1".to_string()));
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_ik_constraint_with_all_fields() {
        let mut ik = IKConstraint::new("ik1".to_string(), 0);
        ik.strength = 0.5;
        ik.target_id = 3;
        ik.invert_direction = true;
        ik.parent_bone_count = 2;
        let props = ik.properties();
        assert_eq!(props.len(), 6);
        let strength = props
            .iter()
            .find(|p| p.key == property_keys::CONSTRAINT_STRENGTH)
            .unwrap();
        assert_eq!(strength.value, PropertyValue::Float(0.5));
        let target = props
            .iter()
            .find(|p| p.key == property_keys::TARGETED_CONSTRAINT_TARGET_ID)
            .unwrap();
        assert_eq!(target.value, PropertyValue::UInt(3));
        let invert = props
            .iter()
            .find(|p| p.key == property_keys::IK_CONSTRAINT_INVERT_DIRECTION)
            .unwrap();
        assert_eq!(invert.value, PropertyValue::UInt(1));
        let pbc = props
            .iter()
            .find(|p| p.key == property_keys::IK_CONSTRAINT_PARENT_BONE_COUNT)
            .unwrap();
        assert_eq!(pbc.value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_distance_constraint_type_key() {
        let dc = DistanceConstraint::new("dc1".to_string(), 0);
        assert_eq!(dc.type_key(), type_keys::DISTANCE_CONSTRAINT);
    }

    #[test]
    fn test_distance_constraint_default_omits_defaults() {
        let dc = DistanceConstraint::new("dc1".to_string(), 0);
        let props = dc.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_distance_constraint_custom_values() {
        let mut dc = DistanceConstraint::new("dc1".to_string(), 0);
        dc.distance = 50.0;
        dc.mode_value = 1;
        dc.target_id = 5;
        let props = dc.properties();
        let dist = props
            .iter()
            .find(|p| p.key == property_keys::DISTANCE_CONSTRAINT_DISTANCE)
            .unwrap();
        assert_eq!(dist.value, PropertyValue::Float(50.0));
        let mode = props
            .iter()
            .find(|p| p.key == property_keys::DISTANCE_CONSTRAINT_MODE_VALUE)
            .unwrap();
        assert_eq!(mode.value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_transform_constraint_type_key() {
        let tc = TransformConstraint::new("tc1".to_string(), 0);
        assert_eq!(tc.type_key(), type_keys::TRANSFORM_CONSTRAINT);
    }

    #[test]
    fn test_transform_constraint_with_origin() {
        let mut tc = TransformConstraint::new("tc1".to_string(), 0);
        tc.origin_x = 0.5;
        tc.origin_y = 0.5;
        tc.target_id = 2;
        let props = tc.properties();
        let ox = props
            .iter()
            .find(|p| p.key == property_keys::TRANSFORM_CONSTRAINT_ORIGIN_X)
            .unwrap();
        assert_eq!(ox.value, PropertyValue::Float(0.5));
        let oy = props
            .iter()
            .find(|p| p.key == property_keys::TRANSFORM_CONSTRAINT_ORIGIN_Y)
            .unwrap();
        assert_eq!(oy.value, PropertyValue::Float(0.5));
    }

    #[test]
    fn test_translation_constraint_type_key() {
        let tlc = TranslationConstraint::new("tl1".to_string(), 0);
        assert_eq!(tlc.type_key(), type_keys::TRANSLATION_CONSTRAINT);
    }

    #[test]
    fn test_translation_constraint_default_omits_defaults() {
        let tlc = TranslationConstraint::new("tl1".to_string(), 0);
        let props = tlc.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_translation_constraint_with_custom_fields() {
        let mut tlc = TranslationConstraint::new("tl1".to_string(), 0);
        tlc.copy_factor = 0.5;
        tlc.offset = true;
        tlc.does_copy = false;
        tlc.min = true;
        tlc.min_value = -100.0;
        tlc.copy_factor_y = 0.75;
        tlc.does_copy_y = false;
        let props = tlc.properties();
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::TRANSFORM_COMPONENT_CONSTRAINT_COPY_FACTOR)
        );
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::TRANSFORM_COMPONENT_CONSTRAINT_OFFSET)
        );
        assert!(props.iter().any(|p| p.key
            == property_keys::TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY
            && p.value == PropertyValue::UInt(0)));
        assert!(props.iter().any(|p| p.key
            == property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_DOES_COPY_Y
            && p.value == PropertyValue::UInt(0)));
    }

    #[test]
    fn test_scale_constraint_type_key() {
        let sc = ScaleConstraint::new("sc1".to_string(), 0);
        assert_eq!(sc.type_key(), type_keys::SCALE_CONSTRAINT);
    }

    #[test]
    fn test_scale_constraint_default_omits_defaults() {
        let sc = ScaleConstraint::new("sc1".to_string(), 0);
        let props = sc.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_rotation_constraint_type_key() {
        let rc = RotationConstraint::new("rc1".to_string(), 0);
        assert_eq!(rc.type_key(), type_keys::ROTATION_CONSTRAINT);
    }

    #[test]
    fn test_rotation_constraint_default_omits_defaults() {
        let rc = RotationConstraint::new("rc1".to_string(), 0);
        let props = rc.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_rotation_constraint_with_custom_fields() {
        let mut rc = RotationConstraint::new("rc1".to_string(), 0);
        rc.strength = 0.8;
        rc.target_id = 4;
        rc.copy_factor = 0.5;
        rc.offset = true;
        rc.does_copy = false;
        rc.min = true;
        rc.max = true;
        rc.min_value = -1.57;
        rc.max_value = 1.57;
        let props = rc.properties();
        assert!(props.len() > 2);
        let strength = props
            .iter()
            .find(|p| p.key == property_keys::CONSTRAINT_STRENGTH)
            .unwrap();
        assert_eq!(strength.value, PropertyValue::Float(0.8));
        assert!(props.iter().any(|p| p.key
            == property_keys::TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY
            && p.value == PropertyValue::UInt(0)));
        assert!(props.iter().any(
            |p| p.key == property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN
                && p.value == PropertyValue::UInt(1)
        ));
        assert!(props.iter().any(
            |p| p.key == property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX
                && p.value == PropertyValue::UInt(1)
        ));
    }
}
