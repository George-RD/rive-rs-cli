use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct ImageAsset {
    pub name: String,
    pub asset_id: u64,
    pub cdn_base_url: String,
}

impl ImageAsset {
    pub fn new(name: String) -> Self {
        Self {
            name,
            asset_id: 0,
            cdn_base_url: String::new(),
        }
    }
}

impl RiveObject for ImageAsset {
    fn type_key(&self) -> u16 {
        type_keys::IMAGE_ASSET
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::ASSET_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if self.asset_id != 0 {
            props.push(Property {
                key: property_keys::FILE_ASSET_ASSET_ID,
                value: PropertyValue::UInt(self.asset_id),
            });
        }
        if !self.cdn_base_url.is_empty() {
            props.push(Property {
                key: property_keys::FILE_ASSET_CDN_BASE_URL,
                value: PropertyValue::String(self.cdn_base_url.clone()),
            });
        }
        props
    }
}

pub struct FontAsset {
    pub name: String,
    pub asset_id: u64,
    pub cdn_base_url: String,
}

impl FontAsset {
    pub fn new(name: String) -> Self {
        Self {
            name,
            asset_id: 0,
            cdn_base_url: String::new(),
        }
    }
}

impl RiveObject for FontAsset {
    fn type_key(&self) -> u16 {
        type_keys::FONT_ASSET
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::ASSET_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if self.asset_id != 0 {
            props.push(Property {
                key: property_keys::FILE_ASSET_ASSET_ID,
                value: PropertyValue::UInt(self.asset_id),
            });
        }
        if !self.cdn_base_url.is_empty() {
            props.push(Property {
                key: property_keys::FILE_ASSET_CDN_BASE_URL,
                value: PropertyValue::String(self.cdn_base_url.clone()),
            });
        }
        props
    }
}

pub struct AudioAsset {
    pub name: String,
    pub asset_id: u64,
    pub cdn_base_url: String,
}

impl AudioAsset {
    pub fn new(name: String) -> Self {
        Self {
            name,
            asset_id: 0,
            cdn_base_url: String::new(),
        }
    }
}

impl RiveObject for AudioAsset {
    fn type_key(&self) -> u16 {
        type_keys::AUDIO_ASSET
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::ASSET_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if self.asset_id != 0 {
            props.push(Property {
                key: property_keys::FILE_ASSET_ASSET_ID,
                value: PropertyValue::UInt(self.asset_id),
            });
        }
        if !self.cdn_base_url.is_empty() {
            props.push(Property {
                key: property_keys::FILE_ASSET_CDN_BASE_URL,
                value: PropertyValue::String(self.cdn_base_url.clone()),
            });
        }
        props
    }
}

pub struct FileAssetContents {
    pub bytes: u64,
}

impl FileAssetContents {
    pub fn new(bytes: u64) -> Self {
        Self { bytes }
    }
}

impl RiveObject for FileAssetContents {
    fn type_key(&self) -> u16 {
        type_keys::FILE_ASSET_CONTENTS
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::FILE_ASSET_CONTENTS_BYTES,
            value: PropertyValue::UInt(self.bytes),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::core::{PropertyValue, property_keys, type_keys};

    #[test]
    fn test_image_asset_type_key() {
        let asset = ImageAsset::new("img1".to_string());
        assert_eq!(asset.type_key(), type_keys::IMAGE_ASSET);
    }

    #[test]
    fn test_image_asset_default_properties() {
        let asset = ImageAsset::new("img1".to_string());
        let props = asset.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].value, PropertyValue::String("img1".to_string()));
        assert_eq!(props[0].key, property_keys::ASSET_NAME);
    }

    #[test]
    fn test_image_asset_with_id_and_cdn() {
        let mut asset = ImageAsset::new("img1".to_string());
        asset.asset_id = 42;
        asset.cdn_base_url = "https://cdn.example.com".to_string();
        let props = asset.properties();
        assert_eq!(props.len(), 3);
        let id_prop = props
            .iter()
            .find(|p| p.key == property_keys::FILE_ASSET_ASSET_ID)
            .unwrap();
        assert_eq!(id_prop.value, PropertyValue::UInt(42));
        let cdn_prop = props
            .iter()
            .find(|p| p.key == property_keys::FILE_ASSET_CDN_BASE_URL)
            .unwrap();
        assert_eq!(
            cdn_prop.value,
            PropertyValue::String("https://cdn.example.com".to_string())
        );
    }

    #[test]
    fn test_font_asset_type_key() {
        let asset = FontAsset::new("font1".to_string());
        assert_eq!(asset.type_key(), type_keys::FONT_ASSET);
    }

    #[test]
    fn test_font_asset_default_properties() {
        let asset = FontAsset::new("font1".to_string());
        let props = asset.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::ASSET_NAME);
    }

    #[test]
    fn test_audio_asset_type_key() {
        let asset = AudioAsset::new("audio1".to_string());
        assert_eq!(asset.type_key(), type_keys::AUDIO_ASSET);
    }

    #[test]
    fn test_audio_asset_default_properties() {
        let asset = AudioAsset::new("audio1".to_string());
        let props = asset.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::ASSET_NAME);
    }

    #[test]
    fn test_file_asset_contents_type_key() {
        let contents = FileAssetContents::new(1024);
        assert_eq!(contents.type_key(), type_keys::FILE_ASSET_CONTENTS);
    }

    #[test]
    fn test_file_asset_contents_properties() {
        let contents = FileAssetContents::new(2048);
        let props = contents.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::FILE_ASSET_CONTENTS_BYTES);
        assert_eq!(props[0].value, PropertyValue::UInt(2048));
    }

    #[test]
    fn test_image_asset_no_parent_id() {
        let asset = ImageAsset::new("img1".to_string());
        let props = asset.properties();
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID)
        );
    }

    #[test]
    fn test_font_asset_no_parent_id() {
        let asset = FontAsset::new("font1".to_string());
        let props = asset.properties();
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID)
        );
    }

    #[test]
    fn test_audio_asset_no_parent_id() {
        let asset = AudioAsset::new("audio1".to_string());
        let props = asset.properties();
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID)
        );
    }
}
