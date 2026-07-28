use super::output::json_error;
use crate::cli::Command;
use rive_cli::{builder, discovery};

pub(super) fn run(command: Command, global_json: bool) {
    match command {
        Command::Schema { compact } => {
            let schema = builder::scene_schema();
            let rendered = if compact {
                serde_json::to_string(&schema)
            } else {
                serde_json::to_string_pretty(&schema)
            };
            match rendered {
                Ok(text) => println!("{}", text),
                Err(e) => {
                    if global_json {
                        json_error(
                            "schema",
                            "encode-failed",
                            format!("JSON serialization failed: {}", e),
                        );
                    }
                    eprintln!("JSON serialization failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Command::Types { category, json } => {
            let json = json || global_json;
            let types = discovery::list_types(category.as_deref());
            if types.is_empty() {
                let category = category.unwrap_or_default();
                let message = format!(
                    "unknown category: {}\nvalid categories: {}",
                    category,
                    discovery::categories().join(", ")
                );
                if json {
                    json_error("types", "unknown-category", message);
                }
                eprintln!("{}", message);
                std::process::exit(1);
            }
            if json {
                match serde_json::to_string_pretty(&types) {
                    Ok(text) => println!("{}", text),
                    Err(e) => json_error(
                        "types",
                        "encode-failed",
                        format!("JSON serialization failed: {}", e),
                    ),
                }
            } else {
                println!("{}", discovery::render_types_text(&types));
            }
        }
        Command::Describe { type_name, json } => {
            let json = json || global_json;
            let Some(description) = discovery::describe(&type_name) else {
                let mut message = format!(
                    "unknown object type: '{}'\nrun `rive-cli types` to list every valid type",
                    type_name
                );
                if let Some(closest) = discovery::closest_type(&type_name) {
                    message.push_str(&format!("\ndid you mean '{}' ?", closest).replace("' ?", "'?"));
                }
                if json {
                    json_error("describe", "unknown-type", message);
                }
                eprintln!("{}", message);
                std::process::exit(1);
            };
            if json {
                match serde_json::to_string_pretty(&description) {
                    Ok(text) => println!("{}", text),
                    Err(e) => json_error(
                        "describe",
                        "encode-failed",
                        format!("JSON serialization failed: {}", e),
                    ),
                }
            } else {
                println!("{}", discovery::render_description_text(&description));
            }
        }
        _ => unreachable!("catalog command router received another command"),
    }
}
