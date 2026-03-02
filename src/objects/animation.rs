use crate::objects::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct LinearAnimation {
    pub name: String,
    pub fps: u64,
    pub duration: u64,
    pub speed: f32,
    pub loop_type: u64,
    pub work_start: u64,
    pub work_end: u64,
    pub enable_work_area: u64,
    pub quantize: u64,
}

impl LinearAnimation {
    pub fn new(name: impl Into<String>, fps: u64, duration: u64) -> Self {
        Self {
            name: name.into(),
            fps,
            duration,
            speed: 1.0,
            loop_type: 0,
            work_start: 0,
            work_end: 0,
            enable_work_area: 0,
            quantize: 0,
        }
    }
}

impl RiveObject for LinearAnimation {
    fn type_key(&self) -> u16 {
        type_keys::LINEAR_ANIMATION
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::ANIMATION_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::LINEAR_ANIMATION_FPS,
                value: PropertyValue::UInt(self.fps),
            },
            Property {
                key: property_keys::LINEAR_ANIMATION_DURATION,
                value: PropertyValue::UInt(self.duration),
            },
        ];

        if self.speed != 1.0 {
            props.push(Property {
                key: property_keys::LINEAR_ANIMATION_SPEED,
                value: PropertyValue::Float(self.speed),
            });
        }

        if self.loop_type != 0 {
            props.push(Property {
                key: property_keys::LINEAR_ANIMATION_LOOP,
                value: PropertyValue::UInt(self.loop_type),
            });
        }

        if self.work_start != 0 {
            props.push(Property {
                key: property_keys::LINEAR_ANIMATION_WORK_START,
                value: PropertyValue::UInt(self.work_start),
            });
        }

        if self.work_end != 0 {
            props.push(Property {
                key: property_keys::LINEAR_ANIMATION_WORK_END,
                value: PropertyValue::UInt(self.work_end),
            });
        }

        if self.enable_work_area != 0 {
            props.push(Property {
                key: property_keys::LINEAR_ANIMATION_ENABLE_WORK_AREA,
                value: PropertyValue::UInt(self.enable_work_area),
            });
        }

        props
    }
}

pub struct KeyedObject {
    pub object_id: u64,
}

impl RiveObject for KeyedObject {
    fn type_key(&self) -> u16 {
        type_keys::KEYED_OBJECT
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::KEYED_OBJECT_ID,
            value: PropertyValue::UInt(self.object_id),
        }]
    }
}

pub struct KeyedProperty {
    pub property_key: u64,
}

impl RiveObject for KeyedProperty {
    fn type_key(&self) -> u16 {
        type_keys::KEYED_PROPERTY
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::KEYED_PROPERTY_KEY,
            value: PropertyValue::UInt(self.property_key),
        }]
    }
}

pub struct KeyFrameDouble {
    pub frame: u64,
    pub interpolation_type: u64,
    pub interpolator_id: u64,
    pub value: f32,
}

impl KeyFrameDouble {
    pub fn new(frame: u64, value: f32) -> Self {
        Self {
            frame,
            interpolation_type: 1,
            interpolator_id: u32::MAX as u64,
            value,
        }
    }
}

impl RiveObject for KeyFrameDouble {
    fn type_key(&self) -> u16 {
        type_keys::KEY_FRAME_DOUBLE
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::KEY_FRAME_FRAME,
                value: PropertyValue::UInt(self.frame),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_TYPE,
                value: PropertyValue::UInt(self.interpolation_type),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_INTERPOLATOR_ID,
                value: PropertyValue::UInt(self.interpolator_id),
            },
            Property {
                key: property_keys::KEY_FRAME_DOUBLE_VALUE,
                value: PropertyValue::Float(self.value),
            },
        ]
    }
}

pub struct KeyFrameColor {
    pub frame: u64,
    pub interpolation_type: u64,
    pub interpolator_id: u64,
    pub value: u32,
}

impl KeyFrameColor {
    pub fn new(frame: u64, value: u32) -> Self {
        Self {
            frame,
            interpolation_type: 1,
            interpolator_id: u32::MAX as u64,
            value,
        }
    }
}

pub struct KeyFrameId {
    pub frame: u64,
    pub interpolation_type: u64,
    pub interpolator_id: u64,
    pub value: u64,
}

pub struct KeyFrameCallback {
    pub frame: u64,
}

impl KeyFrameId {
    pub fn new(frame: u64, value: u64) -> Self {
        Self {
            frame,
            interpolation_type: 1,
            interpolator_id: u32::MAX as u64,
            value,
        }
    }
}

impl RiveObject for KeyFrameId {
    fn type_key(&self) -> u16 {
        type_keys::KEY_FRAME_ID
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::KEY_FRAME_FRAME,
                value: PropertyValue::UInt(self.frame),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_TYPE,
                value: PropertyValue::UInt(self.interpolation_type),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_INTERPOLATOR_ID,
                value: PropertyValue::UInt(self.interpolator_id),
            },
            Property {
                key: property_keys::KEY_FRAME_ID_VALUE,
                value: PropertyValue::UInt(self.value),
            },
        ]
    }
}

impl RiveObject for KeyFrameCallback {
    fn type_key(&self) -> u16 {
        type_keys::KEY_FRAME_CALLBACK
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::KEY_FRAME_FRAME,
            value: PropertyValue::UInt(self.frame),
        }]
    }
}

impl RiveObject for KeyFrameColor {
    fn type_key(&self) -> u16 {
        type_keys::KEY_FRAME_COLOR
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::KEY_FRAME_FRAME,
                value: PropertyValue::UInt(self.frame),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_TYPE,
                value: PropertyValue::UInt(self.interpolation_type),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_INTERPOLATOR_ID,
                value: PropertyValue::UInt(self.interpolator_id),
            },
            Property {
                key: property_keys::KEY_FRAME_COLOR_VALUE,
                value: PropertyValue::Color(self.value),
            },
        ]
    }
}

pub struct Animation {
    pub name: String,
}

impl RiveObject for Animation {
    fn type_key(&self) -> u16 {
        type_keys::ANIMATION
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::ANIMATION_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

pub struct KeyFrame {
    pub frame: u64,
}

impl RiveObject for KeyFrame {
    fn type_key(&self) -> u16 {
        type_keys::KEY_FRAME
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::KEY_FRAME_FRAME,
            value: PropertyValue::UInt(self.frame),
        }]
    }
}

pub struct InterpolatingKeyFrame {
    pub frame: u64,
    pub interpolation_type: u64,
    pub interpolator_id: u64,
}

impl RiveObject for InterpolatingKeyFrame {
    fn type_key(&self) -> u16 {
        type_keys::INTERPOLATING_KEY_FRAME
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::KEY_FRAME_FRAME,
                value: PropertyValue::UInt(self.frame),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_TYPE,
                value: PropertyValue::UInt(self.interpolation_type),
            },
            Property {
                key: property_keys::INTERPOLATING_KEY_FRAME_INTERPOLATOR_ID,
                value: PropertyValue::UInt(self.interpolator_id),
            },
        ]
    }
}

pub struct CubicEaseInterpolator {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

pub struct ElasticInterpolator {
    pub easing_value: u64,
    pub amplitude: f32,
    pub period: f32,
}

impl ElasticInterpolator {
    pub fn new() -> Self {
        Self {
            easing_value: 1,
            amplitude: 1.0,
            period: 1.0,
        }
    }
}

impl CubicEaseInterpolator {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

impl RiveObject for CubicEaseInterpolator {
    fn type_key(&self) -> u16 {
        type_keys::CUBIC_EASE_INTERPOLATOR
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.x1 != 0.42 {
            props.push(Property {
                key: property_keys::CUBIC_INTERPOLATOR_X1,
                value: PropertyValue::Float(self.x1),
            });
        }
        if self.y1 != 0.0 {
            props.push(Property {
                key: property_keys::CUBIC_INTERPOLATOR_Y1,
                value: PropertyValue::Float(self.y1),
            });
        }
        if self.x2 != 0.58 {
            props.push(Property {
                key: property_keys::CUBIC_INTERPOLATOR_X2,
                value: PropertyValue::Float(self.x2),
            });
        }
        if self.y2 != 1.0 {
            props.push(Property {
                key: property_keys::CUBIC_INTERPOLATOR_Y2,
                value: PropertyValue::Float(self.y2),
            });
        }
        props
    }
}

impl RiveObject for ElasticInterpolator {
    fn type_key(&self) -> u16 {
        type_keys::ELASTIC_INTERPOLATOR
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.easing_value != 1 {
            props.push(Property {
                key: property_keys::ELASTIC_EASING_VALUE,
                value: PropertyValue::UInt(self.easing_value),
            });
        }
        if self.amplitude != 1.0 {
            props.push(Property {
                key: property_keys::ELASTIC_AMPLITUDE,
                value: PropertyValue::Float(self.amplitude),
            });
        }
        if self.period != 1.0 {
            props.push(Property {
                key: property_keys::ELASTIC_PERIOD,
                value: PropertyValue::Float(self.period),
            });
        }
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_animation_type_key() {
        let anim = LinearAnimation::new("test", 24, 60);
        assert_eq!(anim.type_key(), 31);
    }

    #[test]
    fn test_linear_animation_properties() {
        let anim = LinearAnimation::new("walk", 24, 120);
        let props = anim.properties();
        assert_eq!(props.len(), 3);
        let name_prop = props
            .iter()
            .find(|p| p.key == property_keys::ANIMATION_NAME)
            .unwrap();
        assert_eq!(name_prop.value, PropertyValue::String("walk".to_string()));
        let fps_prop = props
            .iter()
            .find(|p| p.key == property_keys::LINEAR_ANIMATION_FPS)
            .unwrap();
        assert_eq!(fps_prop.value, PropertyValue::UInt(24));
        let dur_prop = props
            .iter()
            .find(|p| p.key == property_keys::LINEAR_ANIMATION_DURATION)
            .unwrap();
        assert_eq!(dur_prop.value, PropertyValue::UInt(120));
        assert!(
            props
                .iter()
                .all(|p| p.key != property_keys::LINEAR_ANIMATION_SPEED)
        );
        assert!(
            props
                .iter()
                .all(|p| p.key != property_keys::LINEAR_ANIMATION_QUANTIZE)
        );
    }

    #[test]
    fn test_linear_animation_no_extra_props() {
        let anim = LinearAnimation::new("test", 24, 60);
        let props = anim.properties();
        assert!(props.iter().all(|p| p.key != 4));
    }

    #[test]
    fn test_keyed_object_type_key() {
        let ko = KeyedObject { object_id: 3 };
        assert_eq!(ko.type_key(), 25);
    }

    #[test]
    fn test_keyed_object_properties() {
        let ko = KeyedObject { object_id: 7 };
        let props = ko.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::KEYED_OBJECT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(7));
        assert!(props.iter().all(|p| p.key != 4));
        assert!(props.iter().all(|p| p.key != 5));
    }

    #[test]
    fn test_keyed_property_type_key() {
        let kp = KeyedProperty { property_key: 13 };
        assert_eq!(kp.type_key(), 26);
    }

    #[test]
    fn test_keyed_property_properties() {
        let kp = KeyedProperty { property_key: 13 };
        let props = kp.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::KEYED_PROPERTY_KEY);
        assert_eq!(props[0].value, PropertyValue::UInt(13));
        assert!(props.iter().all(|p| p.key != 4));
        assert!(props.iter().all(|p| p.key != 5));
    }

    #[test]
    fn test_key_frame_double_type_key() {
        let kf = KeyFrameDouble::new(10, 100.0);
        assert_eq!(kf.type_key(), 30);
    }

    #[test]
    fn test_key_frame_double_properties() {
        let kf = KeyFrameDouble::new(10, 100.0);
        let props = kf.properties();
        let frame_prop = props
            .iter()
            .find(|p| p.key == property_keys::KEY_FRAME_FRAME)
            .unwrap();
        assert_eq!(frame_prop.value, PropertyValue::UInt(10));
        let val_prop = props
            .iter()
            .find(|p| p.key == property_keys::KEY_FRAME_DOUBLE_VALUE)
            .unwrap();
        assert_eq!(val_prop.value, PropertyValue::Float(100.0));
        let interp_prop = props
            .iter()
            .find(|p| p.key == property_keys::INTERPOLATING_KEY_FRAME_TYPE)
            .unwrap();
        assert_eq!(interp_prop.value, PropertyValue::UInt(1));
        assert!(props.iter().all(|p| p.key != 4));
        assert!(props.iter().all(|p| p.key != 5));
    }

    #[test]
    fn test_key_frame_color_type_key() {
        let kf = KeyFrameColor::new(5, 0xFF0000FF);
        assert_eq!(kf.type_key(), 37);
    }

    #[test]
    fn test_key_frame_color_properties() {
        let kf = KeyFrameColor::new(5, 0xFF0000FF);
        let props = kf.properties();
        let frame_prop = props
            .iter()
            .find(|p| p.key == property_keys::KEY_FRAME_FRAME)
            .unwrap();
        assert_eq!(frame_prop.value, PropertyValue::UInt(5));
        let val_prop = props
            .iter()
            .find(|p| p.key == property_keys::KEY_FRAME_COLOR_VALUE)
            .unwrap();
        assert_eq!(val_prop.value, PropertyValue::Color(0xFF0000FF));
        assert!(props.iter().all(|p| p.key != 4));
        assert!(props.iter().all(|p| p.key != 5));
    }

    #[test]
    fn test_key_frame_id_type_key() {
        let kf = KeyFrameId::new(12, 42);
        assert_eq!(kf.type_key(), type_keys::KEY_FRAME_ID);
    }

    #[test]
    fn test_key_frame_id_properties() {
        let kf = KeyFrameId::new(12, 42);
        let props = kf.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[0].key, property_keys::KEY_FRAME_FRAME);
        assert_eq!(props[0].value, PropertyValue::UInt(12));
        assert_eq!(props[1].key, property_keys::INTERPOLATING_KEY_FRAME_TYPE);
        assert_eq!(props[1].value, PropertyValue::UInt(1));
        assert_eq!(
            props[2].key,
            property_keys::INTERPOLATING_KEY_FRAME_INTERPOLATOR_ID
        );
        assert_eq!(props[2].value, PropertyValue::UInt(u32::MAX as u64));
        assert_eq!(props[3].key, property_keys::KEY_FRAME_ID_VALUE);
        assert_eq!(props[3].value, PropertyValue::UInt(42));
    }

    #[test]
    fn test_key_frame_callback_type_key() {
        let kf = KeyFrameCallback { frame: 7 };
        assert_eq!(kf.type_key(), type_keys::KEY_FRAME_CALLBACK);
    }

    #[test]
    fn test_key_frame_callback_properties() {
        let kf = KeyFrameCallback { frame: 7 };
        let props = kf.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::KEY_FRAME_FRAME);
        assert_eq!(props[0].value, PropertyValue::UInt(7));
    }

    #[test]
    fn test_cubic_ease_interpolator_type_key() {
        let interp = CubicEaseInterpolator::new(0.42, 0.0, 0.58, 1.0);
        assert_eq!(interp.type_key(), type_keys::CUBIC_EASE_INTERPOLATOR);
    }

    #[test]
    fn test_cubic_ease_interpolator_default_properties() {
        let interp = CubicEaseInterpolator::new(0.42, 0.0, 0.58, 1.0);
        let props = interp.properties();
        assert_eq!(props.len(), 0);
    }

    #[test]
    fn test_cubic_ease_interpolator_custom_properties() {
        let interp = CubicEaseInterpolator::new(0.0, 0.0, 1.0, 1.0);
        let props = interp.properties();
        assert_eq!(props.len(), 2);
        let x1 = props
            .iter()
            .find(|p| p.key == property_keys::CUBIC_INTERPOLATOR_X1)
            .unwrap();
        assert_eq!(x1.value, PropertyValue::Float(0.0));
        let x2 = props
            .iter()
            .find(|p| p.key == property_keys::CUBIC_INTERPOLATOR_X2)
            .unwrap();
        assert_eq!(x2.value, PropertyValue::Float(1.0));
    }

    #[test]
    fn test_cubic_ease_interpolator_all_custom() {
        let interp = CubicEaseInterpolator::new(0.25, 0.1, 0.25, 1.0);
        let props = interp.properties();
        assert_eq!(props.len(), 3);
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::CUBIC_INTERPOLATOR_X1)
        );
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::CUBIC_INTERPOLATOR_Y1)
        );
    }

    #[test]
    fn test_cubic_ease_interpolator_no_name_no_parent() {
        let interp = CubicEaseInterpolator::new(0.0, 0.0, 1.0, 1.0);
        let props = interp.properties();
        assert!(props.iter().all(|p| p.key != property_keys::COMPONENT_NAME));
        assert!(
            props
                .iter()
                .all(|p| p.key != property_keys::COMPONENT_PARENT_ID)
        );
    }

    #[test]
    fn test_elastic_interpolator_type_key() {
        let interp = ElasticInterpolator::new();
        assert_eq!(interp.type_key(), type_keys::ELASTIC_INTERPOLATOR);
    }

    #[test]
    fn test_elastic_interpolator_default_properties() {
        let interp = ElasticInterpolator::new();
        let props = interp.properties();
        assert_eq!(props.len(), 0);
    }

    #[test]
    fn test_elastic_interpolator_custom_properties() {
        let interp = ElasticInterpolator {
            easing_value: 2,
            amplitude: 1.5,
            period: 0.8,
        };
        let props = interp.properties();
        assert_eq!(props.len(), 3);
        assert!(props.iter().any(|p| {
            p.key == property_keys::ELASTIC_EASING_VALUE && p.value == PropertyValue::UInt(2)
        }));
        assert!(props.iter().any(|p| {
            p.key == property_keys::ELASTIC_AMPLITUDE && p.value == PropertyValue::Float(1.5)
        }));
        assert!(props.iter().any(|p| {
            p.key == property_keys::ELASTIC_PERIOD && p.value == PropertyValue::Float(0.8)
        }));
    }

    #[test]
    fn test_key_frame_double_default_interpolator_sentinel() {
        let kf = KeyFrameDouble::new(0, 0.0);
        assert_eq!(kf.interpolator_id, u32::MAX as u64);
    }

    #[test]
    fn test_key_frame_color_default_interpolator_sentinel() {
        let kf = KeyFrameColor::new(0, 0);
        assert_eq!(kf.interpolator_id, u32::MAX as u64);
    }

    #[test]
    fn test_key_frame_id_default_interpolator_sentinel() {
        let kf = KeyFrameId::new(0, 0);
        assert_eq!(kf.interpolator_id, u32::MAX as u64);
    }
}
