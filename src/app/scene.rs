use super::output::{json_error, json_success};
use crate::cli::Command;
use rive_cli::{builder, compile, scaffold, validator};

pub(super) fn run(command: Command, global_json: bool) {
    match command {
        Command::Generate {
            input,
            output,
            file_id,
            json,
        } => {
            let json = json || global_json;
            let json_str = std::fs::read_to_string(&input).unwrap_or_else(|e| {
                if json {
                    json_error(
                        "generate",
                        "read-failed",
                        format!("error reading {:?}: {}", input, e),
                    );
                }
                eprintln!("error reading {:?}: {}", input, e);
                std::process::exit(1);
            });
            let spec = serde_json::from_str::<builder::SceneSpec>(&json_str).unwrap_or_else(|e| {
                if json {
                    json_error(
                        "generate",
                        "parse-failed",
                        format!("error parsing JSON: {}", e),
                    );
                }
                eprintln!("error parsing JSON: {}", e);
                std::process::exit(1);
            });
            let base_dir = input
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or_else(|| std::path::Path::new("."));
            let bytes =
                compile::compile_scene(&spec, Some(base_dir), file_id).unwrap_or_else(|e| {
                    if json {
                        json_error("generate", e.code(), format!("invalid scene spec: {}", e));
                    }
                    eprintln!("invalid scene spec: {}", e);
                    std::process::exit(1);
                });
            std::fs::write(&output, &bytes).unwrap_or_else(|e| {
                if json {
                    json_error(
                        "generate",
                        "write-failed",
                        format!("error writing {:?}: {}", output, e),
                    );
                }
                eprintln!("error writing {:?}: {}", output, e);
                std::process::exit(1);
            });
            if json {
                #[derive(serde::Serialize)]
                struct GenerateOutput {
                    bytes_written: usize,
                    output_path: String,
                }
                let result = GenerateOutput {
                    bytes_written: bytes.len(),
                    output_path: output.display().to_string(),
                };
                json_success("generate", &result);
            } else {
                eprintln!("wrote {} bytes to {:?}", bytes.len(), output);
            }
        }
        Command::New {
            template,
            list,
            output,
        } => {
            if list {
                if global_json {
                    json_success(
                        "new",
                        &serde_json::json!({"templates": scaffold::templates()}),
                    );
                } else {
                    for info in scaffold::templates() {
                        println!("{}: {}", info.name, info.description);
                    }
                }
                return;
            }
            let Some(template) = template else {
                if global_json {
                    json_error("new", "usage", "a template is required");
                }
                eprintln!("a template is required; use `rive-cli new --list`");
                std::process::exit(1);
            };
            let scene = scaffold::template_json(&template).unwrap_or_else(|_| {
                if global_json {
                    json_error(
                        "new",
                        "unknown-template",
                        format!("unknown template '{}'; use `rive-cli new --list`", template),
                    );
                }
                eprintln!("unknown template '{}'; use `rive-cli new --list`", template);
                std::process::exit(1);
            });
            if let Some(output) = output {
                std::fs::write(&output, scene).unwrap_or_else(|e| {
                    if global_json {
                        json_error(
                            "new",
                            "write-failed",
                            format!("error writing {:?}: {}", output, e),
                        );
                    }
                    eprintln!("error writing {:?}: {}", output, e);
                    std::process::exit(1);
                });
                if global_json {
                    #[derive(serde::Serialize)]
                    struct NewOutput {
                        template: String,
                        output_path: String,
                    }
                    json_success(
                        "new",
                        &NewOutput {
                            template,
                            output_path: output.display().to_string(),
                        },
                    );
                } else {
                    eprintln!("wrote {} scene template to {:?}", template, output);
                }
            } else {
                println!("{}", scene);
            }
        }
        Command::Validate { file, json } => {
            let json = json || global_json;
            let bytes = std::fs::read(&file).unwrap_or_else(|e| {
                if json {
                    json_error(
                        "validate",
                        "read-failed",
                        format!("error reading {:?}: {}", file, e),
                    );
                }
                eprintln!("error reading {:?}: {}", file, e);
                std::process::exit(1);
            });
            match validator::validate_riv(&bytes) {
                Ok(report) => {
                    if json {
                        if !report.valid {
                            json_error(
                                "validate",
                                "invalid-riv",
                                format!("invalid ({} errors)", report.errors.len()),
                            );
                        }
                        json_success("validate", &report);
                    } else {
                        println!(
                            "RIVE v{}.{} file_id={}",
                            report.header.major_version,
                            report.header.minor_version,
                            report.header.file_id
                        );
                        println!("{} objects", report.object_count);
                        for warning in &report.warnings {
                            eprintln!("warning: {}", warning);
                        }
                        if report.valid {
                            println!("valid");
                        } else {
                            for err in &report.errors {
                                eprintln!("error: {}", err);
                            }
                            eprintln!("invalid ({} errors)", report.errors.len());
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    if json {
                        json_error("validate", "invalid-riv", format!("invalid: {}", e));
                    }
                    eprintln!("invalid: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Command::Inspect {
            file,
            json,
            artboard_index,
            artboard_name,
            local_index,
            type_key,
            type_name,
            object_index,
            property_key,
        } => {
            let bytes = std::fs::read(&file).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {}", file, e);
                std::process::exit(1);
            });
            let filter = validator::InspectFilter {
                artboard_indices: artboard_index,
                artboard_names: artboard_name,
                local_indices: local_index,
                type_keys: type_key,
                type_names: type_name,
                object_indices: object_index,
                property_keys: property_key,
            };
            if json {
                match validator::parse_riv(&bytes, &filter) {
                    Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
                        Ok(json_str) => println!("{}", json_str),
                        Err(e) => {
                            eprintln!("JSON serialization failed: {}", e);
                            std::process::exit(1);
                        }
                    },
                    Err(e) => {
                        eprintln!("parse failed: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                match validator::inspect_riv(&bytes, &filter) {
                    Ok(output) => {
                        print!("{}", output);
                    }
                    Err(e) => {
                        eprintln!("inspect failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Command::Decompile { file } => {
            let bytes = std::fs::read(&file).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {}", file, e);
                std::process::exit(1);
            });
            match validator::parse_riv(&bytes, &validator::InspectFilter::default()) {
                Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
                    Ok(json_str) => println!("{}", json_str),
                    Err(e) => {
                        eprintln!("JSON serialization failed: {}", e);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("decompile failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!("scene command router received another command"),
    }
}
