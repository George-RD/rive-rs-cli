use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct Text {
    pub name: String,
    pub parent_id: u64,
    pub align_value: u64,
    pub sizing_value: u64,
    pub overflow_value: u64,
    pub width: f32,
    pub height: f32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub paragraph_spacing: f32,
    pub origin_value: u64,
}

impl Text {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            align_value: 0,
            sizing_value: 0,
            overflow_value: 0,
            width: 0.0,
            height: 0.0,
            origin_x: 0.0,
            origin_y: 0.0,
            paragraph_spacing: 0.0,
            origin_value: 0,
        }
    }
}

impl RiveObject for Text {
    fn type_key(&self) -> u16 {
        type_keys::TEXT
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
        if self.align_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_ALIGN_VALUE,
                value: PropertyValue::UInt(self.align_value),
            });
        }
        if self.sizing_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_SIZING_VALUE,
                value: PropertyValue::UInt(self.sizing_value),
            });
        }
        if self.overflow_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_OVERFLOW_VALUE,
                value: PropertyValue::UInt(self.overflow_value),
            });
        }
        if self.width != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_WIDTH,
                value: PropertyValue::Float(self.width),
            });
        }
        if self.height != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_HEIGHT,
                value: PropertyValue::Float(self.height),
            });
        }
        if self.origin_x != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_ORIGIN_X,
                value: PropertyValue::Float(self.origin_x),
            });
        }
        if self.origin_y != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_ORIGIN_Y,
                value: PropertyValue::Float(self.origin_y),
            });
        }
        if self.paragraph_spacing != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_PARAGRAPH_SPACING,
                value: PropertyValue::Float(self.paragraph_spacing),
            });
        }
        if self.origin_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_ORIGIN_VALUE,
                value: PropertyValue::UInt(self.origin_value),
            });
        }
        props
    }
}

pub struct TextStyle {
    pub name: String,
    pub parent_id: u64,
    pub font_size: f32,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub font_asset_id: u64,
}

impl TextStyle {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            font_size: 12.0,
            line_height: -1.0,
            letter_spacing: 0.0,
            font_asset_id: u32::MAX as u64,
        }
    }
}

impl RiveObject for TextStyle {
    fn type_key(&self) -> u16 {
        type_keys::TEXT_STYLE
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
        if self.font_size != 12.0 {
            props.push(Property {
                key: property_keys::TEXT_STYLE_FONT_SIZE,
                value: PropertyValue::Float(self.font_size),
            });
        }
        if self.line_height != -1.0 {
            props.push(Property {
                key: property_keys::TEXT_STYLE_LINE_HEIGHT,
                value: PropertyValue::Float(self.line_height),
            });
        }
        if self.letter_spacing != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_STYLE_LETTER_SPACING,
                value: PropertyValue::Float(self.letter_spacing),
            });
        }
        if self.font_asset_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TEXT_STYLE_FONT_ASSET_ID,
                value: PropertyValue::UInt(self.font_asset_id),
            });
        }
        props
    }
}

pub struct TextValueRun {
    pub name: String,
    pub parent_id: u64,
    pub style_id: u64,
    pub text: String,
}

impl TextValueRun {
    pub fn new(name: String, parent_id: u64, text: String) -> Self {
        Self {
            name,
            parent_id,
            style_id: u32::MAX as u64,
            text,
        }
    }
}

impl RiveObject for TextValueRun {
    fn type_key(&self) -> u16 {
        type_keys::TEXT_VALUE_RUN
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
            Property {
                key: property_keys::TEXT_VALUE_RUN_TEXT,
                value: PropertyValue::String(self.text.clone()),
            },
        ];
        if self.style_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::TEXT_VALUE_RUN_STYLE_ID,
                value: PropertyValue::UInt(self.style_id),
            });
        }
        props
    }
}

pub struct TextModifierRange {
    pub parent_id: u64,
    pub units_value: u64,
    pub type_value: u64,
    pub mode_value: u64,
    pub modify_from: f32,
    pub modify_to: f32,
    pub strength: f32,
    pub clamp: bool,
    pub falloff_from: f32,
    pub falloff_to: f32,
    pub offset: f32,
    pub run_id: u64,
}

impl TextModifierRange {
    pub fn new(parent_id: u64) -> Self {
        Self {
            parent_id,
            units_value: 0,
            type_value: 0,
            mode_value: 0,
            modify_from: 0.0,
            modify_to: 1.0,
            strength: 1.0,
            clamp: false,
            falloff_from: 0.0,
            falloff_to: 0.0,
            offset: 0.0,
            run_id: 0,
        }
    }
}

impl RiveObject for TextModifierRange {
    fn type_key(&self) -> u16 {
        type_keys::TEXT_MODIFIER_RANGE
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::with_capacity(12);
        props.push(Property {
            key: property_keys::COMPONENT_PARENT_ID,
            value: PropertyValue::UInt(self.parent_id),
        });
        if self.units_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_UNITS_VALUE,
                value: PropertyValue::UInt(self.units_value),
            });
        }
        if self.type_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_TYPE_VALUE,
                value: PropertyValue::UInt(self.type_value),
            });
        }
        if self.mode_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_MODE_VALUE,
                value: PropertyValue::UInt(self.mode_value),
            });
        }
        if self.modify_from != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_MODIFY_FROM,
                value: PropertyValue::Float(self.modify_from),
            });
        }
        if self.modify_to != 1.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_MODIFY_TO,
                value: PropertyValue::Float(self.modify_to),
            });
        }
        if self.strength != 1.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.clamp {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_CLAMP,
                value: PropertyValue::Bool(self.clamp),
            });
        }
        if self.falloff_from != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_FALLOFF_FROM,
                value: PropertyValue::Float(self.falloff_from),
            });
        }
        if self.falloff_to != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_FALLOFF_TO,
                value: PropertyValue::Float(self.falloff_to),
            });
        }
        if self.offset != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_OFFSET,
                value: PropertyValue::Float(self.offset),
            });
        }
        if self.run_id != 0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_RANGE_RUN_ID,
                value: PropertyValue::UInt(self.run_id),
            });
        }
        props
    }
}

pub struct TextModifierGroup {
    pub name: String,
    pub parent_id: u64,
    pub modifier_flags: u64,
    pub origin_x: f32,
    pub origin_y: f32,
    pub opacity: f32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl TextModifierGroup {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            modifier_flags: 0,
            origin_x: 0.0,
            origin_y: 0.0,
            opacity: 1.0,
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

impl RiveObject for TextModifierGroup {
    fn type_key(&self) -> u16 {
        type_keys::TEXT_MODIFIER_GROUP
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::with_capacity(11);
        props.push(Property {
            key: property_keys::COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        });
        props.push(Property {
            key: property_keys::COMPONENT_PARENT_ID,
            value: PropertyValue::UInt(self.parent_id),
        });
        if self.modifier_flags != 0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_MODIFIER_FLAGS,
                value: PropertyValue::UInt(self.modifier_flags),
            });
        }
        if self.origin_x != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_ORIGIN_X,
                value: PropertyValue::Float(self.origin_x),
            });
        }
        if self.origin_y != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_ORIGIN_Y,
                value: PropertyValue::Float(self.origin_y),
            });
        }
        if self.opacity != 1.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_OPACITY,
                value: PropertyValue::Float(self.opacity),
            });
        }
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        if self.rotation != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_ROTATION,
                value: PropertyValue::Float(self.rotation),
            });
        }
        if self.scale_x != 1.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_SCALE_X,
                value: PropertyValue::Float(self.scale_x),
            });
        }
        if self.scale_y != 1.0 {
            props.push(Property {
                key: property_keys::TEXT_MODIFIER_GROUP_SCALE_Y,
                value: PropertyValue::Float(self.scale_y),
            });
        }
        props
    }
}

pub struct TextVariationModifier {
    pub parent_id: u64,
    pub axis_tag: u64,
    pub axis_value: f32,
}

impl RiveObject for TextVariationModifier {
    fn type_key(&self) -> u16 {
        type_keys::TEXT_VARIATION_MODIFIER
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::COMPONENT_PARENT_ID,
            value: PropertyValue::UInt(self.parent_id),
        }];
        if self.axis_tag != 0 {
            props.push(Property {
                key: property_keys::TEXT_VARIATION_MODIFIER_AXIS_TAG,
                value: PropertyValue::UInt(self.axis_tag),
            });
        }
        if self.axis_value != 0.0 {
            props.push(Property {
                key: property_keys::TEXT_VARIATION_MODIFIER_AXIS_VALUE,
                value: PropertyValue::Float(self.axis_value),
            });
        }
        props
    }
}

pub struct TextStyleFeature {
    pub parent_id: u64,
    pub tag: u64,
    pub feature_value: u64,
}

impl RiveObject for TextStyleFeature {
    fn type_key(&self) -> u16 {
        type_keys::TEXT_STYLE_FEATURE
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::COMPONENT_PARENT_ID,
            value: PropertyValue::UInt(self.parent_id),
        }];
        if self.tag != 0 {
            props.push(Property {
                key: property_keys::TEXT_STYLE_FEATURE_TAG,
                value: PropertyValue::UInt(self.tag),
            });
        }
        if self.feature_value != 0 {
            props.push(Property {
                key: property_keys::TEXT_STYLE_FEATURE_FEATURE_VALUE,
                value: PropertyValue::UInt(self.feature_value),
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
    fn test_text_type_key() {
        let text = Text::new("text1".to_string(), 0);
        assert_eq!(text.type_key(), type_keys::TEXT);
    }

    #[test]
    fn test_text_default_properties() {
        let text = Text::new("text1".to_string(), 1);
        let props = text.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].value, PropertyValue::String("text1".to_string()));
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_text_with_dimensions() {
        let mut text = Text::new("text1".to_string(), 0);
        text.width = 200.0;
        text.height = 100.0;
        text.align_value = 1;
        let props = text.properties();
        assert_eq!(props.len(), 5);
        let width_prop = props
            .iter()
            .find(|p| p.key == property_keys::TEXT_WIDTH)
            .unwrap();
        assert_eq!(width_prop.value, PropertyValue::Float(200.0));
        let height_prop = props
            .iter()
            .find(|p| p.key == property_keys::TEXT_HEIGHT)
            .unwrap();
        assert_eq!(height_prop.value, PropertyValue::Float(100.0));
        let align_prop = props
            .iter()
            .find(|p| p.key == property_keys::TEXT_ALIGN_VALUE)
            .unwrap();
        assert_eq!(align_prop.value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_text_style_type_key() {
        let style = TextStyle::new("style1".to_string(), 0);
        assert_eq!(style.type_key(), type_keys::TEXT_STYLE);
    }

    #[test]
    fn test_text_style_default_properties() {
        let style = TextStyle::new("style1".to_string(), 0);
        let props = style.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_text_style_custom_font_size() {
        let mut style = TextStyle::new("style1".to_string(), 0);
        style.font_size = 24.0;
        style.letter_spacing = 1.5;
        let props = style.properties();
        assert_eq!(props.len(), 4);
        let font_size_prop = props
            .iter()
            .find(|p| p.key == property_keys::TEXT_STYLE_FONT_SIZE)
            .unwrap();
        assert_eq!(font_size_prop.value, PropertyValue::Float(24.0));
        let spacing_prop = props
            .iter()
            .find(|p| p.key == property_keys::TEXT_STYLE_LETTER_SPACING)
            .unwrap();
        assert_eq!(spacing_prop.value, PropertyValue::Float(1.5));
    }

    #[test]
    fn test_text_style_with_font_asset_id() {
        let mut style = TextStyle::new("style1".to_string(), 0);
        style.font_asset_id = 5;
        let props = style.properties();
        let font_prop = props
            .iter()
            .find(|p| p.key == property_keys::TEXT_STYLE_FONT_ASSET_ID)
            .unwrap();
        assert_eq!(font_prop.value, PropertyValue::UInt(5));
    }

    #[test]
    fn test_text_value_run_type_key() {
        let run = TextValueRun::new("run1".to_string(), 0, "Hello".to_string());
        assert_eq!(run.type_key(), type_keys::TEXT_VALUE_RUN);
    }

    #[test]
    fn test_text_value_run_default_properties() {
        let run = TextValueRun::new("run1".to_string(), 1, "Hello World".to_string());
        let props = run.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].value, PropertyValue::String("run1".to_string()));
        assert_eq!(props[1].value, PropertyValue::UInt(1));
        assert_eq!(
            props[2].value,
            PropertyValue::String("Hello World".to_string())
        );
    }

    #[test]
    fn test_text_value_run_with_style_id() {
        let mut run = TextValueRun::new("run1".to_string(), 0, "text".to_string());
        run.style_id = 2;
        let props = run.properties();
        assert_eq!(props.len(), 4);
        let style_prop = props
            .iter()
            .find(|p| p.key == property_keys::TEXT_VALUE_RUN_STYLE_ID)
            .unwrap();
        assert_eq!(style_prop.value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_text_modifier_range_default_properties() {
        let range = TextModifierRange::new(7);
        assert_eq!(range.type_key(), type_keys::TEXT_MODIFIER_RANGE);
        let props = range.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(7));
    }

    #[test]
    fn test_text_modifier_range_custom_properties() {
        let mut range = TextModifierRange::new(7);
        range.modify_to = 0.5;
        range.strength = 0.25;
        range.clamp = true;
        range.run_id = 3;
        let props = range.properties();
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_MODIFIER_RANGE_MODIFY_TO
                && property.value == PropertyValue::Float(0.5)
        }));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_MODIFIER_RANGE_STRENGTH
                && property.value == PropertyValue::Float(0.25)
        }));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_MODIFIER_RANGE_CLAMP
                && property.value == PropertyValue::Bool(true)
        }));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_MODIFIER_RANGE_RUN_ID
                && property.value == PropertyValue::UInt(3)
        }));
    }

    #[test]
    fn test_text_modifier_group_default_properties() {
        let group = TextModifierGroup::new("modifier".to_string(), 7);
        assert_eq!(group.type_key(), type_keys::TEXT_MODIFIER_GROUP);
        let props = group.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
    }

    #[test]
    fn test_text_modifier_group_custom_properties() {
        let mut group = TextModifierGroup::new("modifier".to_string(), 7);
        group.opacity = 0.5;
        group.scale_x = 1.5;
        group.scale_y = 0.75;
        let props = group.properties();
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_MODIFIER_GROUP_OPACITY
                && property.value == PropertyValue::Float(0.5)
        }));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_MODIFIER_GROUP_SCALE_X
                && property.value == PropertyValue::Float(1.5)
        }));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_MODIFIER_GROUP_SCALE_Y
                && property.value == PropertyValue::Float(0.75)
        }));
    }

    #[test]
    fn test_text_variation_modifier_properties() {
        let modifier = TextVariationModifier {
            parent_id: 9,
            axis_tag: 0x77676874,
            axis_value: 700.0,
        };
        assert_eq!(modifier.type_key(), type_keys::TEXT_VARIATION_MODIFIER);
        let props = modifier.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(9));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_VARIATION_MODIFIER_AXIS_TAG
                && property.value == PropertyValue::UInt(0x77676874)
        }));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_VARIATION_MODIFIER_AXIS_VALUE
                && property.value == PropertyValue::Float(700.0)
        }));
    }

    #[test]
    fn test_text_style_feature_properties() {
        let feature = TextStyleFeature {
            parent_id: 5,
            tag: 0x6C696761,
            feature_value: 1,
        };
        assert_eq!(feature.type_key(), type_keys::TEXT_STYLE_FEATURE);
        let props = feature.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(5));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_STYLE_FEATURE_TAG
                && property.value == PropertyValue::UInt(0x6C696761)
        }));
        assert!(props.iter().any(|property| {
            property.key == property_keys::TEXT_STYLE_FEATURE_FEATURE_VALUE
                && property.value == PropertyValue::UInt(1)
        }));
    }
}
