use crate::cli::{AiCommand, Command};
use rive_cli::ai;

pub(super) fn run(command: Command) {
    match command {
        Command::Ai { command } => match command {
            AiCommand::Generate {
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
            AiCommand::Lab {
                suite,
                output_dir,
                file_id,
                max_retries,
                baseline,
                write_baseline,
                model,
                provider,
                json,
            } => {
                match ai::run_eval_suite_configured(
                    &suite,
                    &output_dir,
                    file_id,
                    max_retries,
                    baseline.as_deref(),
                    write_baseline.as_deref(),
                    model,
                    provider,
                ) {
                    Ok(report) => {
                        if json {
                            let json_str =
                                serde_json::to_string_pretty(&report).unwrap_or_else(|e| {
                                    eprintln!("JSON serialization failed: {}", e);
                                    std::process::exit(1);
                                });
                            println!("{}", json_str);
                        } else {
                            println!("run_id={}", report.run_id);
                            println!("output_dir={}", report.output_dir);
                            println!("provider={}", report.provider);
                            println!("model={}", report.model);
                            println!(
                                "validity_rate={:.3} ({}/{})",
                                report.validity_rate, report.valid_count, report.case_count
                            );
                            println!("average_retries={:.3}", report.average_retries);
                            println!("trait_adherence_rate={:.3}", report.trait_adherence_rate);
                            println!(
                                "pipeline_reproducibility_rate={:.3}",
                                report.pipeline_reproducibility_rate
                            );
                            println!("drift_count={}", report.drift_count);
                            println!("passed={}", report.passed);
                        }
                        if !report.passed {
                            if report.drift_count > 0 {
                                eprintln!(
                                    "regression drift detected in {} case(s)",
                                    report.drift_count
                                );
                            }
                            for failure in &report.gate_failures {
                                eprintln!("evaluation gate failed: {}", failure);
                            }
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
        _ => unreachable!("AI command router received another command"),
    }
}
