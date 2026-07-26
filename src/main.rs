mod cli;

use clap::Parser;
#[cfg(feature = "mcp")]
use rive_cli::mcp;
use rive_cli::{ai, builder, discovery, encoder, objects, render, scaffold, validator};
fn json_error(command: &str, code: &str, message: impl std::fmt::Display) -> ! {
    let envelope = serde_json::json!({
        "ok": false,
        "command": command,
        "code": code,
        "message": message.to_string(),
    });
    eprintln!("{}", envelope);
    std::process::exit(1);
}

fn json_success<T: serde::Serialize>(command: &str, value: &T) {
    let mut output = serde_json::to_value(value).unwrap_or_else(|e| {
        json_error(
            command,
            "encode-failed",
            format!("JSON serialization failed: {}", e),
        );
    });
    if let Some(object) = output.as_object_mut() {
        object.insert("ok".to_owned(), serde_json::Value::Bool(true));
    }
    match serde_json::to_string_pretty(&output) {
        Ok(text) => println!("{}", text),
        Err(e) => json_error(
            command,
            "encode-failed",
            format!("JSON serialization failed: {}", e),
        ),
    }
}

fn main() {
    let command_line: Vec<String> = std::env::args().collect();
    let json_requested = command_line.iter().any(|argument| argument == "--json");
    let json_command = command_line
        .iter()
        .find_map(|argument| match argument.as_str() {
            "generate" | "new" | "validate" | "inspect" | "decompile" | "render" | "schema"
            | "types" | "describe" | "ai" => Some(argument.as_str()),
            _ => None,
        })
        .unwrap_or("cli");
    let cli = cli::Cli::try_parse().unwrap_or_else(|error| {
        if json_requested
            && !matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            )
        {
            json_error(json_command, "usage", error);
        }
        error.exit();
    });

    if cli.list_presets {
        if cli.json {
            let json =
                serde_json::to_string_pretty(&builder::artboard_presets()).unwrap_or_else(|e| {
                    eprintln!("JSON serialization failed: {}", e);
                    std::process::exit(1);
                });
            println!("{}", json);
        } else {
            for preset in builder::artboard_presets() {
                println!("{}: {}x{}", preset.name, preset.width, preset.height);
            }
        }
        return;
    }

    #[cfg(feature = "mcp")]
    if cli.mcp {
        mcp::run_server();
        return;
    }

    let command = cli.command.unwrap_or_else(|| {
        if cli.json {
            json_error("cli", "usage", "no command provided");
        }
        eprintln!("no command provided");
        std::process::exit(1);
    });

    let global_json = cli.json;
    match command {
        cli::Command::Generate {
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
            let scene = builder::build_scene(&spec).unwrap_or_else(|e| {
                if json {
                    json_error(
                        "generate",
                        "invalid-scene",
                        format!("invalid scene spec: {}", e),
                    );
                }
                eprintln!("invalid scene spec: {}", e);
                std::process::exit(1);
            });
            let refs: Vec<&dyn objects::core::RiveObject> = scene.iter().map(|o| &**o).collect();
            let bytes = encoder::encode_riv(&refs, file_id);
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
        cli::Command::New {
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
        cli::Command::Validate { file, json } => {
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
        cli::Command::Inspect {
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
        cli::Command::Decompile { file } => {
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
        cli::Command::Render {
            file,
            output,
            frames,
            fps,
            animation,
            state_machine,
            inputs,
            artboard,
            width,
            height,
            scale,
            background,
            contact_sheet,
            preview,
            browser,
            json,
        } => {
            let json = json || global_json;
            let bytes = std::fs::read(&file).unwrap_or_else(|e| {
                if json {
                    json_error(
                        "render",
                        "read-failed",
                        format!("error reading {:?}: {}", file, e),
                    );
                }
                eprintln!("error reading {:?}: {}", file, e);
                std::process::exit(1);
            });
            let frame_list = render::parse_frame_spec(&frames).unwrap_or_else(|e| {
                if json {
                    json_error("render", "usage", format!("invalid --frames value: {}", e));
                }
                eprintln!("invalid --frames value: {}", e);
                std::process::exit(1);
            });
            let options = render::RenderOptions {
                riv: bytes,
                source_path: file.clone(),
                output_dir: output,
                frames: frame_list,
                fps,
                animation,
                state_machine,
                inputs,
                artboard,
                width,
                height,
                scale,
                background,
                contact_sheet,
                preview,
                browser,
            };
            match render::render(&options) {
                Ok(manifest) => {
                    if json {
                        match serde_json::to_string_pretty(&manifest) {
                            Ok(text) => println!("{}", text),
                            Err(e) => json_error(
                                "render",
                                "encode-failed",
                                format!("JSON serialization failed: {}", e),
                            ),
                        }
                    } else {
                        println!("{}", render::render_manifest_text(&manifest));
                    }
                    if manifest.frames.iter().all(|frame| frame.blank) {
                        eprintln!(
                            "warning: every rendered frame is a single flat color; the artboard may be empty or the shapes may be off-screen"
                        );
                    }
                }
                Err(e) => {
                    if json {
                        json_error("render", "render-failed", e);
                    }
                    eprintln!("render failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Command::Schema { compact } => {
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
        cli::Command::Types { category, json } => {
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
        cli::Command::Describe { type_name, json } => {
            let json = json || global_json;
            let Some(description) = discovery::describe(&type_name) else {
                let mut message = format!(
                    "unknown object type: '{}'\nrun `rive-cli types` to list every valid type",
                    type_name
                );
                if let Some(closest) = discovery::closest_type(&type_name) {
                    message.push_str(&format!("\ndid you mean '{}'?", closest));
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
        cli::Command::Ai { command } => match command {
            cli::AiCommand::Generate {
                prompt,
                template,
                output,
                file_id,
                dry_run,
                model,
                provider: provider_name,
                max_retries,
                json,
            } => {
                let config = ai::AiConfig::resolve(model, provider_name).unwrap_or_else(|e| {
                    eprintln!("AI config error: {}", e);
                    std::process::exit(1);
                });
                let input = if let Some(ref t) = template {
                    t.clone()
                } else {
                    prompt.unwrap()
                };
                let provider =
                    ai::create_provider(&config, template.is_some()).unwrap_or_else(|e| {
                        eprintln!("AI provider error: {}", e);
                        std::process::exit(1);
                    });
                let scene_json = provider.generate(&input, &config).unwrap_or_else(|e| {
                    eprintln!("AI generation error: {}", e);
                    std::process::exit(1);
                });
                if dry_run {
                    let pretty = serde_json::to_string_pretty(&scene_json).unwrap_or_else(|e| {
                        eprintln!("failed to serialize scene JSON: {}", e);
                        std::process::exit(1);
                    });
                    println!("{}", pretty);
                    return;
                }

                let engine = ai::RepairEngine::new(max_retries);
                match engine.repair(scene_json, file_id) {
                    Ok(result) => {
                        let bytes = result.riv_bytes;
                        let attempts = result.attempts;
                        let total_retries = result.total_retries;

                        if total_retries > 0 && !json {
                            eprintln!("repair succeeded after {} retry(ies)", total_retries);
                            let summary = ai::format_repair_summary(&attempts);
                            eprint!("{}", summary);
                        }
                        std::fs::write(&output, &bytes).unwrap_or_else(|e| {
                            eprintln!("error writing {:?}: {}", output, e);
                            std::process::exit(1);
                        });
                        if json {
                            #[derive(serde::Serialize)]
                            struct AiGenerateOutput<'a> {
                                output_path: String,
                                bytes_written: usize,
                                retries: u8,
                                attempts: &'a [ai::RepairAttempt],
                            }
                            let json_result = AiGenerateOutput {
                                output_path: output.display().to_string(),
                                bytes_written: bytes.len(),
                                retries: total_retries,
                                attempts: &attempts,
                            };
                            let json_str = serde_json::to_string_pretty(&json_result)
                                .unwrap_or_else(|e| {
                                    eprintln!("JSON serialization failed: {}", e);
                                    std::process::exit(1);
                                });
                            println!("{}", json_str);
                        } else {
                            eprintln!("wrote {} bytes to {:?}", bytes.len(), output);
                        }
                    }
                    Err(e) => {
                        if let ai::AiError::RepairFailed { ref attempts, .. } = e {
                            let summary = ai::format_repair_summary(attempts);
                            eprint!("{}", summary);
                            let hints = ai::remediation_hints(attempts);
                            if !hints.is_empty() {
                                eprintln!("hints:");
                                for hint in &hints {
                                    eprintln!("  - {}", hint);
                                }
                            }
                        }
                        eprintln!("repair failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            cli::AiCommand::Lab {
                suite,
                output_dir,
                file_id,
                max_retries,
                baseline,
                write_baseline,
                json,
            } => {
                match ai::run_eval_suite(
                    &suite,
                    &output_dir,
                    file_id,
                    max_retries,
                    baseline.as_deref(),
                    write_baseline.as_deref(),
                ) {
                    Ok(report) => {
                        if json {
                            let json_str =
                                serde_json::to_string_pretty(&report).unwrap_or_else(|e| {
                                    eprintln!("JSON serialization failed: {}", e);
                                    std::process::exit(1);
                                });
                            println!("{}", json_str);
                            if report.drift_count > 0 {
                                eprintln!(
                                    "regression drift detected in {} case(s)",
                                    report.drift_count
                                );
                                std::process::exit(1);
                            }
                        } else {
                            println!("run_id={}", report.run_id);
                            println!("output_dir={}", report.output_dir);
                            println!(
                                "validity_rate={:.3} ({}/{})",
                                report.validity_rate, report.valid_count, report.case_count
                            );
                            println!("average_retries={:.3}", report.average_retries);
                            println!("style_adherence_rate={:.3}", report.style_adherence_rate);
                            println!("reproducibility_rate={:.3}", report.reproducibility_rate);
                            println!("drift_count={}", report.drift_count);
                            if report.drift_count > 0 {
                                eprintln!(
                                    "regression drift detected in {} case(s)",
                                    report.drift_count
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("ai lab failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
    }
}
