use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct LayoutComponent {
    pub name: String,
    pub parent_id: u64,
    pub clip: bool,
    pub width: f32,
    pub height: f32,
    pub style_id: u64,
    pub fractional_width: f32,
    pub fractional_height: f32,
}

impl LayoutComponent {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            clip: false,
            width: 0.0,
            height: 0.0,
            style_id: 0,
            fractional_width: 1.0,
            fractional_height: 1.0,
        }
    }
}

impl RiveObject for LayoutComponent {
    fn type_key(&self) -> u16 {
        type_keys::LAYOUT_COMPONENT
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
        if self.clip {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_CLIP,
                value: PropertyValue::UInt(1),
            });
        }
        if self.width != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_WIDTH,
                value: PropertyValue::Float(self.width),
            });
        }
        if self.height != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_HEIGHT,
                value: PropertyValue::Float(self.height),
            });
        }
        if self.style_id != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_STYLE_ID,
                value: PropertyValue::UInt(self.style_id),
            });
        }
        if self.fractional_width != 1.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_FRACTIONAL_WIDTH,
                value: PropertyValue::Float(self.fractional_width),
            });
        }
        if self.fractional_height != 1.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_FRACTIONAL_HEIGHT,
                value: PropertyValue::Float(self.fractional_height),
            });
        }
        props
    }
}

pub struct LayoutComponentStyle {
    pub name: String,
    pub parent_id: u64,
    pub gap_horizontal: f32,
    pub gap_vertical: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub min_width: f32,
    pub min_height: f32,
    pub border_left: f32,
    pub border_right: f32,
    pub border_top: f32,
    pub border_bottom: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub padding_left: f32,
    pub padding_right: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
    pub position_left: f32,
    pub position_right: f32,
    pub position_top: f32,
    pub position_bottom: f32,
    pub flex_direction: u64,
    pub flex_wrap: u64,
    pub align_items: u64,
    pub align_content: u64,
    pub justify_content: u64,
    pub display: u64,
    pub position_type: u64,
    pub overflow: u64,
    pub intrinsically_sized: bool,
    pub width_units: u64,
    pub height_units: u64,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: f32,
    pub aspect_ratio: f32,
}

impl LayoutComponentStyle {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            gap_horizontal: 0.0,
            gap_vertical: 0.0,
            max_width: 0.0,
            max_height: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            border_left: 0.0,
            border_right: 0.0,
            border_top: 0.0,
            border_bottom: 0.0,
            margin_left: 0.0,
            margin_right: 0.0,
            margin_top: 0.0,
            margin_bottom: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            padding_top: 0.0,
            padding_bottom: 0.0,
            position_left: 0.0,
            position_right: 0.0,
            position_top: 0.0,
            position_bottom: 0.0,
            flex_direction: 0,
            flex_wrap: 0,
            align_items: 0,
            align_content: 0,
            justify_content: 0,
            display: 0,
            position_type: 0,
            overflow: 0,
            intrinsically_sized: false,
            width_units: 0,
            height_units: 0,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: 0.0,
            aspect_ratio: 0.0,
        }
    }
}

impl RiveObject for LayoutComponentStyle {
    fn type_key(&self) -> u16 {
        type_keys::LAYOUT_COMPONENT_STYLE
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
        if self.gap_horizontal != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_GAP_HORIZONTAL,
                value: PropertyValue::Float(self.gap_horizontal),
            });
        }
        if self.gap_vertical != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_GAP_VERTICAL,
                value: PropertyValue::Float(self.gap_vertical),
            });
        }
        if self.max_width != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MAX_WIDTH,
                value: PropertyValue::Float(self.max_width),
            });
        }
        if self.max_height != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MAX_HEIGHT,
                value: PropertyValue::Float(self.max_height),
            });
        }
        if self.min_width != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MIN_WIDTH,
                value: PropertyValue::Float(self.min_width),
            });
        }
        if self.min_height != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MIN_HEIGHT,
                value: PropertyValue::Float(self.min_height),
            });
        }
        if self.border_left != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_BORDER_LEFT,
                value: PropertyValue::Float(self.border_left),
            });
        }
        if self.border_right != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_BORDER_RIGHT,
                value: PropertyValue::Float(self.border_right),
            });
        }
        if self.border_top != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_BORDER_TOP,
                value: PropertyValue::Float(self.border_top),
            });
        }
        if self.border_bottom != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_BORDER_BOTTOM,
                value: PropertyValue::Float(self.border_bottom),
            });
        }
        if self.margin_left != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MARGIN_LEFT,
                value: PropertyValue::Float(self.margin_left),
            });
        }
        if self.margin_right != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MARGIN_RIGHT,
                value: PropertyValue::Float(self.margin_right),
            });
        }
        if self.margin_top != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MARGIN_TOP,
                value: PropertyValue::Float(self.margin_top),
            });
        }
        if self.margin_bottom != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_MARGIN_BOTTOM,
                value: PropertyValue::Float(self.margin_bottom),
            });
        }
        if self.padding_left != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_PADDING_LEFT,
                value: PropertyValue::Float(self.padding_left),
            });
        }
        if self.padding_right != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_PADDING_RIGHT,
                value: PropertyValue::Float(self.padding_right),
            });
        }
        if self.padding_top != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_PADDING_TOP,
                value: PropertyValue::Float(self.padding_top),
            });
        }
        if self.padding_bottom != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_PADDING_BOTTOM,
                value: PropertyValue::Float(self.padding_bottom),
            });
        }
        if self.position_left != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_POSITION_LEFT,
                value: PropertyValue::Float(self.position_left),
            });
        }
        if self.position_right != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_POSITION_RIGHT,
                value: PropertyValue::Float(self.position_right),
            });
        }
        if self.position_top != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_POSITION_TOP,
                value: PropertyValue::Float(self.position_top),
            });
        }
        if self.position_bottom != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_POSITION_BOTTOM,
                value: PropertyValue::Float(self.position_bottom),
            });
        }
        if self.flex_direction != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_FLEX_DIRECTION,
                value: PropertyValue::UInt(self.flex_direction),
            });
        }
        if self.flex_wrap != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_FLEX_WRAP,
                value: PropertyValue::UInt(self.flex_wrap),
            });
        }
        if self.align_items != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_ALIGN_ITEMS,
                value: PropertyValue::UInt(self.align_items),
            });
        }
        if self.align_content != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_ALIGN_CONTENT,
                value: PropertyValue::UInt(self.align_content),
            });
        }
        if self.justify_content != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_JUSTIFY_CONTENT,
                value: PropertyValue::UInt(self.justify_content),
            });
        }
        if self.display != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_DISPLAY,
                value: PropertyValue::UInt(self.display),
            });
        }
        if self.position_type != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_POSITION_TYPE,
                value: PropertyValue::UInt(self.position_type),
            });
        }
        if self.overflow != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_OVERFLOW,
                value: PropertyValue::UInt(self.overflow),
            });
        }
        if self.intrinsically_sized {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_INTRINSICALLY_SIZED_VALUE,
                value: PropertyValue::UInt(1),
            });
        }
        if self.width_units != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_WIDTH_UNITS,
                value: PropertyValue::UInt(self.width_units),
            });
        }
        if self.height_units != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_HEIGHT_UNITS,
                value: PropertyValue::UInt(self.height_units),
            });
        }
        if self.flex_grow != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_FLEX_GROW,
                value: PropertyValue::Float(self.flex_grow),
            });
        }
        if self.flex_shrink != 1.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_FLEX_SHRINK,
                value: PropertyValue::Float(self.flex_shrink),
            });
        }
        if self.flex_basis != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_FLEX_BASIS,
                value: PropertyValue::Float(self.flex_basis),
            });
        }
        if self.aspect_ratio != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_STYLE_ASPECT_RATIO,
                value: PropertyValue::Float(self.aspect_ratio),
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
    fn test_layout_component_type_key() {
        let lc = LayoutComponent::new("lc1".to_string(), 0);
        assert_eq!(lc.type_key(), type_keys::LAYOUT_COMPONENT);
    }

    #[test]
    fn test_layout_component_default_properties() {
        let lc = LayoutComponent::new("lc1".to_string(), 1);
        let props = lc.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].value, PropertyValue::String("lc1".to_string()));
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_layout_component_with_dimensions() {
        let mut lc = LayoutComponent::new("lc1".to_string(), 0);
        lc.width = 300.0;
        lc.height = 200.0;
        lc.clip = true;
        let props = lc.properties();
        assert_eq!(props.len(), 5);
        let clip_prop = props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_COMPONENT_CLIP)
            .unwrap();
        assert_eq!(clip_prop.value, PropertyValue::UInt(1));
        let width_prop = props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_COMPONENT_WIDTH)
            .unwrap();
        assert_eq!(width_prop.value, PropertyValue::Float(300.0));
    }

    #[test]
    fn test_layout_component_fractional_defaults() {
        let lc = LayoutComponent::new("lc1".to_string(), 0);
        let props = lc.properties();
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::LAYOUT_COMPONENT_FRACTIONAL_WIDTH)
        );
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::LAYOUT_COMPONENT_FRACTIONAL_HEIGHT)
        );
    }

    #[test]
    fn test_layout_component_style_type_key() {
        let style = LayoutComponentStyle::new("style1".to_string(), 0);
        assert_eq!(style.type_key(), type_keys::LAYOUT_COMPONENT_STYLE);
    }

    #[test]
    fn test_layout_component_style_default_properties() {
        let style = LayoutComponentStyle::new("style1".to_string(), 0);
        let props = style.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_layout_component_style_with_padding() {
        let mut style = LayoutComponentStyle::new("style1".to_string(), 0);
        style.padding_left = 10.0;
        style.padding_right = 10.0;
        style.padding_top = 5.0;
        style.padding_bottom = 5.0;
        let props = style.properties();
        assert_eq!(props.len(), 6);
        let pl = props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_STYLE_PADDING_LEFT)
            .unwrap();
        assert_eq!(pl.value, PropertyValue::Float(10.0));
    }

    #[test]
    fn test_layout_component_style_flex_properties() {
        let mut style = LayoutComponentStyle::new("style1".to_string(), 0);
        style.flex_direction = 1;
        style.justify_content = 2;
        style.align_items = 3;
        style.flex_grow = 1.0;
        style.flex_shrink = 0.0;
        let props = style.properties();
        assert_eq!(props.len(), 7);
        let fd = props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_STYLE_FLEX_DIRECTION)
            .unwrap();
        assert_eq!(fd.value, PropertyValue::UInt(1));
        let fs = props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_STYLE_FLEX_SHRINK)
            .unwrap();
        assert_eq!(fs.value, PropertyValue::Float(0.0));
    }

    #[test]
    fn test_layout_component_style_intrinsically_sized() {
        let mut style = LayoutComponentStyle::new("style1".to_string(), 0);
        style.intrinsically_sized = true;
        let props = style.properties();
        let is_prop = props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_STYLE_INTRINSICALLY_SIZED_VALUE)
            .unwrap();
        assert_eq!(is_prop.value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_layout_component_style_flex_shrink_default() {
        let style = LayoutComponentStyle::new("style1".to_string(), 0);
        let props = style.properties();
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::LAYOUT_STYLE_FLEX_SHRINK)
        );
    }
}
