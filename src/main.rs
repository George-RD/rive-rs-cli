#![allow(dead_code, unused_imports)]

mod ai;
mod builder;
mod cli;
mod encoder;
mod objects;
mod validator;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    if cli.list_presets {
        for preset in builder::artboard_presets() {
            println!("{}: {}x{}", preset.name, preset.width, preset.height);
        }
        return;
    }

    let command = cli.command.unwrap_or_else(|| {
        eprintln!("no command provided");
        std::process::exit(1);
    });

    match command {
        cli::Command::Generate {
            input,
            output,
            file_id,
        } => {
            let json_str = std::fs::read_to_string(&input).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {}", input, e);
                std::process::exit(1);
            });
            let spec = serde_json::from_str::<builder::SceneSpec>(&json_str).unwrap_or_else(|e| {
                eprintln!("error parsing JSON: {}", e);
                std::process::exit(1);
            });
            let scene = builder::build_scene(&spec).unwrap_or_else(|e| {
                eprintln!("invalid scene spec: {}", e);
                std::process::exit(1);
            });
            let refs: Vec<&dyn objects::core::RiveObject> = scene.iter().map(|o| &**o).collect();
            let bytes = encoder::encode_riv(&refs, file_id);
            std::fs::write(&output, &bytes).unwrap_or_else(|e| {
                eprintln!("error writing {:?}: {}", output, e);
                std::process::exit(1);
            });
            eprintln!("wrote {} bytes to {:?}", bytes.len(), output);
        }
        cli::Command::Validate { file } => {
            let bytes = std::fs::read(&file).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {}", file, e);
                std::process::exit(1);
            });
            match validator::validate_riv(&bytes) {
                Ok(report) => {
                    println!(
                        "RIVE v{}.{} file_id={}",
                        report.header.major_version,
                        report.header.minor_version,
                        report.header.file_id
                    );
                    println!("{} objects", report.object_count);
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
                Err(e) => {
                    eprintln!("invalid: {}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Command::Inspect {
            file,
            json,
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
            } => {
                let config = ai::AiConfig::resolve(model, provider_name).unwrap_or_else(|e| {
                    eprintln!("AI config error: {}", e);
                    std::process::exit(1);
                });
                if template.is_some() && prompt.is_some() {
                    eprintln!("error: cannot use both --template and --prompt");
                    std::process::exit(1);
                }
                let input = if let Some(ref t) = template {
                    t.clone()
                } else if let Some(ref p) = prompt {
                    p.clone()
                } else {
                    eprintln!("error: provide --prompt or --template");
                    std::process::exit(1);
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
                        if result.total_retries > 0 {
                            eprintln!("repair succeeded after {} retry(ies)", result.total_retries);
                            let summary = ai::format_repair_summary(&result.attempts);
                            eprint!("{}", summary);
                        }
                        let bytes = result.riv_bytes;
                        std::fs::write(&output, &bytes).unwrap_or_else(|e| {
                            eprintln!("error writing {:?}: {}", output, e);
                            std::process::exit(1);
                        });
                        eprintln!("wrote {} bytes to {:?}", bytes.len(), output);
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
                    Err(e) => {
                        eprintln!("ai lab failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
    }
}
