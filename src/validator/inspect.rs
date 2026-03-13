use std::collections::BTreeSet;

use crate::objects::core::{property_keys, type_keys};
use crate::objects::generated_registry;

use super::parser::{ParsedRiv, PropertyValueRead, RivObject};

#[derive(Debug, Clone, Default)]
pub struct InspectFilter {
    pub artboard_indices: Vec<usize>,
    pub artboard_names: Vec<String>,
    pub local_indices: Vec<usize>,
    pub type_keys: Vec<u16>,
    pub type_names: Vec<String>,
    pub object_indices: Vec<usize>,
    pub property_keys: Vec<u16>,
}

impl InspectFilter {
    pub(crate) fn is_active(&self) -> bool {
        !self.artboard_indices.is_empty()
            || !self.artboard_names.is_empty()
            || !self.local_indices.is_empty()
            || !self.type_keys.is_empty()
            || !self.type_names.is_empty()
            || !self.object_indices.is_empty()
            || !self.property_keys.is_empty()
    }
}

fn type_name(key: u16) -> &'static str {
    generated_registry::type_name(key).unwrap_or("Unknown")
}

fn component_name(object: &RivObject) -> Option<String> {
    object
        .properties
        .iter()
        .find(|property| property.key == property_keys::COMPONENT_NAME)
        .and_then(|property| match &property.value {
            PropertyValueRead::String(name) => Some(name.clone()),
            _ => None,
        })
}

pub(crate) fn annotate_object_context(objects: &mut [RivObject]) {
    let mut current_artboard_index = None;
    let mut current_artboard_name = None;
    let mut next_artboard_index = 0;
    let mut next_local_index = 0;

    for object in objects.iter_mut() {
        if object.type_key == type_keys::ARTBOARD {
            let artboard_name = component_name(object);
            object.artboard_index = Some(next_artboard_index);
            object.artboard_name = artboard_name.clone();
            object.local_index = Some(0);

            current_artboard_index = Some(next_artboard_index);
            current_artboard_name = artboard_name;
            next_artboard_index += 1;
            next_local_index = 1;
            continue;
        }

        object.artboard_index = current_artboard_index;
        object.artboard_name = current_artboard_name.clone();
        if current_artboard_index.is_some() {
            object.local_index = Some(next_local_index);
            next_local_index += 1;
        }
    }
}

fn matches_object_filter(filter: &InspectFilter, object: &RivObject) -> bool {
    let artboard_index_match = filter.artboard_indices.is_empty()
        || object
            .artboard_index
            .is_some_and(|index| filter.artboard_indices.contains(&index));
    let artboard_name_match = filter.artboard_names.is_empty()
        || object.artboard_name.as_ref().is_some_and(|object_name| {
            let object_name = object_name.to_lowercase();
            filter
                .artboard_names
                .iter()
                .any(|name| name.to_lowercase() == object_name)
        });
    let local_index_match = filter.local_indices.is_empty()
        || object
            .local_index
            .is_some_and(|index| filter.local_indices.contains(&index));
    let type_key_match = filter.type_keys.is_empty() || filter.type_keys.contains(&object.type_key);
    let type_name_match = filter.type_names.is_empty()
        || filter
            .type_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(type_name(object.type_key)));
    let index_match =
        filter.object_indices.is_empty() || filter.object_indices.contains(&object.object_index);

    artboard_index_match
        && artboard_name_match
        && local_index_match
        && type_key_match
        && type_name_match
        && index_match
}

pub(crate) fn apply_inspect_filter(mut parsed: ParsedRiv, filter: &InspectFilter) -> ParsedRiv {
    let mut filtered_objects = Vec::new();

    for mut object in parsed.objects {
        if !matches_object_filter(filter, &object) {
            continue;
        }

        if !filter.property_keys.is_empty() {
            object
                .properties
                .retain(|property| filter.property_keys.contains(&property.key));
        }

        filtered_objects.push(object);
    }

    parsed.objects = filtered_objects;
    parsed
}

pub fn inspect_riv(data: &[u8], filter: &InspectFilter) -> Result<String, String> {
    let parsed = super::parser::parse_riv(data, filter)?;
    let mut out = std::string::String::new();

    let artboard_count = parsed
        .objects
        .iter()
        .filter_map(|object| object.artboard_index)
        .collect::<BTreeSet<_>>()
        .len();

    out.push_str(&format!(
        "RIVE v{}.{} file_id={}\n",
        parsed.header.major_version, parsed.header.minor_version, parsed.header.file_id
    ));
    out.push_str(&format!(
        "ToC: {} properties\n",
        parsed.toc_property_keys.len()
    ));
    out.push_str(&format!("Objects: {}\n", parsed.objects.len()));
    if parsed.objects.is_empty() && filter.is_active() {
        out.push_str("No objects matched the provided filters.\n");
        return Ok(out);
    }
    if artboard_count > 1 {
        out.push_str(&format!("Artboards: {}\n", artboard_count));
    }

    let show_artboard_sections = artboard_count > 1
        || (filter.is_active()
            && artboard_count == 1
            && parsed
                .objects
                .iter()
                .all(|object| object.artboard_index.is_some()));
    let mut current_artboard = None;
    for obj in &parsed.objects {
        if show_artboard_sections && obj.artboard_index != current_artboard {
            if let Some(artboard_index) = obj.artboard_index {
                out.push_str(&format!(
                    "--- Artboard {} ({}) ---\n",
                    artboard_index,
                    obj.artboard_name.as_deref().unwrap_or("unnamed")
                ));
            }
            current_artboard = obj.artboard_index;
        }
        if let Some(local_index) = obj.local_index {
            out.push_str(&format!(
                "[{}:{}] type={} ({})\n",
                obj.object_index,
                local_index,
                obj.type_key,
                type_name(obj.type_key)
            ));
        } else {
            out.push_str(&format!(
                "[{}] type={} ({})\n",
                obj.object_index,
                obj.type_key,
                type_name(obj.type_key)
            ));
        }
        for prop in &obj.properties {
            let val_str = match &prop.value {
                PropertyValueRead::UInt(v) => format!("uint({})", v),
                PropertyValueRead::String(v) => format!("string({:?})", v),
                PropertyValueRead::Float(v) => format!("float({})", v),
                PropertyValueRead::Color(v) => format!("color(0x{:08X})", v),
            };
            out.push_str(&format!(
                "  {}({}) {}\n",
                prop.name.as_deref().unwrap_or("?"),
                prop.key,
                val_str
            ));
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::encode_riv;
    use crate::objects::artboard::{Artboard, Backboard};
    use crate::objects::shapes::Shape;

    #[test]
    fn test_inspect_riv_minimal() {
        let backboard = Backboard;
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let data = encode_riv(&[&backboard, &artboard], 0);
        let output = inspect_riv(&data, &InspectFilter::default()).unwrap();

        assert!(output.contains("Backboard"));
        assert!(output.contains("Artboard"));
        assert!(output.contains("RIVE v7.0"));
        assert!(output.contains("Objects: 2"));
    }

    #[test]
    fn test_inspect_riv_uses_global_and_local_indices_for_filtered_multi_artboard_output() {
        let backboard = Backboard;
        let first_artboard = Artboard::new("Screen A".to_string(), 400.0, 300.0);
        let first_shape = Shape::new("shape_a".to_string(), 0);
        let second_artboard = Artboard::new("Screen B".to_string(), 800.0, 600.0);
        let second_shape = Shape::new("shape_b".to_string(), 0);
        let data = encode_riv(
            &[
                &backboard,
                &first_artboard,
                &first_shape,
                &second_artboard,
                &second_shape,
            ],
            0,
        );
        let filter = InspectFilter {
            artboard_indices: vec![1],
            local_indices: vec![1],
            ..InspectFilter::default()
        };
        let output = inspect_riv(&data, &filter).unwrap();

        assert!(output.contains("Artboard: Screen B") || output.contains("Artboard 1 (Screen B)"));
        assert!(output.contains("[4:1] type=3 (Shape)"));
        assert!(!output.contains("[0] type=23 (Backboard)"));
    }
}
