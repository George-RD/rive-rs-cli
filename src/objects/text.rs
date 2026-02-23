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
}
