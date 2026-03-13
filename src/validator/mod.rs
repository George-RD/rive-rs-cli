mod binary_reader;
mod inspect;
mod parser;

pub use binary_reader::BinaryReader;
pub use inspect::*;
pub use parser::*;

use std::collections::HashMap;

use serde::Serialize;

use crate::objects::core::{property_keys, type_keys};

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub header: RivHeader,
    pub object_count: usize,
    pub type_counts: HashMap<u16, usize>,
    pub errors: Vec<String>,
    pub valid: bool,
}

pub fn validate_riv(data: &[u8]) -> Result<ValidationReport, String> {
    let parsed = parse_riv(data, &InspectFilter::default())?;

    let mut type_counts: HashMap<u16, usize> = HashMap::new();
    for obj in &parsed.objects {
        *type_counts.entry(obj.type_key).or_insert(0) += 1;
    }

    let mut errors: Vec<String> = Vec::new();

    if !type_counts.contains_key(&23) {
        errors.push("missing Backboard (type 23)".to_string());
    }

    if !type_counts.contains_key(&1) {
        errors.push("missing Artboard (type 1)".to_string());
    }

    if !parsed.objects.is_empty() && parsed.objects[0].type_key != 23 {
        errors.push(format!(
            "first object should be Backboard (type 23), got type {}",
            parsed.objects[0].type_key
        ));
    }

    let mut image_assets_seen: u64 = 0;
    for (idx, obj) in parsed.objects.iter().enumerate() {
        if obj.type_key == type_keys::IMAGE_ASSET {
            image_assets_seen += 1;
            continue;
        }
        if obj.type_key == type_keys::IMAGE {
            let asset_id = obj
                .properties
                .iter()
                .find(|p| p.key == property_keys::IMAGE_ASSET_ID)
                .and_then(|p| match p.value {
                    PropertyValueRead::UInt(v) => Some(v),
                    _ => None,
                });

            match asset_id {
                Some(v) if v < image_assets_seen => {}
                Some(v) => errors.push(format!(
                    "image object at index {} references image asset index {} but only {} image asset(s) were defined before it",
                    idx, v, image_assets_seen
                )),
                None => errors.push(format!(
                    "image object at index {} is missing image asset reference property {}",
                    idx,
                    property_keys::IMAGE_ASSET_ID
                )),
            }
        }
    }

    let valid = errors.is_empty();

    Ok(ValidationReport {
        header: parsed.header,
        object_count: parsed.objects.len(),
        type_counts,
        errors,
        valid,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::encode_riv;
    use crate::objects::artboard::{Artboard, Backboard};
    use crate::objects::assets::ImageAsset;
    use crate::objects::shapes::Image;

    #[test]
    fn test_validate_riv_minimal() {
        let backboard = Backboard;
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let data = encode_riv(&[&backboard, &artboard], 0);
        let report = validate_riv(&data).unwrap();

        assert!(report.valid);
        assert!(report.errors.is_empty());
        assert_eq!(report.object_count, 2);
        assert_eq!(report.type_counts.get(&23), Some(&1));
        assert_eq!(report.type_counts.get(&1), Some(&1));
    }

    #[test]
    fn test_validate_riv_image_reference_requires_asset_before_image() {
        let backboard = Backboard;
        let artboard = Artboard::new("Main".to_string(), 500.0, 500.0);
        let image = Image::new("Hero".to_string(), 0, 0);
        let data = encode_riv(&[&backboard, &artboard, &image], 0);
        let report = validate_riv(&data).unwrap();

        assert!(!report.valid);
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.contains("references image asset index")),
            "expected image asset reference error, got: {:?}",
            report.errors
        );
    }

    #[test]
    fn test_validate_riv_image_reference_with_preceding_asset() {
        let backboard = Backboard;
        let artboard = Artboard::new("Main".to_string(), 500.0, 500.0);
        let image_asset = ImageAsset::new("HeroAsset".to_string());
        let image = Image::new("Hero".to_string(), 0, 0);
        let data = encode_riv(&[&backboard, &image_asset, &artboard, &image], 0);
        let report = validate_riv(&data).unwrap();

        assert!(report.valid, "unexpected errors: {:?}", report.errors);
    }
}
