use super::output::{json_error, json_success};
use crate::cli::{AuthoringCommand, Command};
use rive_cli::authoring::{self, AuthoringError};
use rive_cli::{builder, compile};

pub(super) fn run(command: Command, global_json: bool) {
    let Command::Authoring { command } = command else {
        unreachable!("authoring dispatcher received a non-authoring command");
    };

    match command {
        AuthoringCommand::Compile {
            input,
            output,
            file_id,
            json,
        } => {
            let json = json || global_json;
            let input_text = std::fs::read_to_string(&input).unwrap_or_else(|error| {
                if json {
                    json_error(
                        "authoring",
                        "read-failed",
                        format!("error reading {:?}: {error}", input),
                    );
                }
                eprintln!("error reading {:?}: {error}", input);
                std::process::exit(1);
            });
            let lowered = authoring::lower_authoring_json(&input_text)
                .unwrap_or_else(|error| exit_lowering_error(error, json));
            let authoring::LoweredAuthoring { scene, source_map } = lowered;
            let scene =
                serde_json::from_value::<builder::SceneSpec>(scene).unwrap_or_else(|error| {
                    if json {
                        json_error(
                            "authoring",
                            "parse-failed",
                            format!("lowered SceneSpec could not be parsed: {error}"),
                        );
                    }
                    eprintln!("lowered SceneSpec could not be parsed: {error}");
                    std::process::exit(1);
                });
            let base_dir = input
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or_else(|| std::path::Path::new("."));
            let bytes =
                compile::compile_scene(&scene, Some(base_dir), file_id).unwrap_or_else(|error| {
                    if json {
                        json_error(
                            "authoring",
                            error.code(),
                            format!("invalid lowered SceneSpec: {error}"),
                        );
                    }
                    eprintln!("invalid lowered SceneSpec: {error}");
                    std::process::exit(1);
                });
            std::fs::write(&output, &bytes).unwrap_or_else(|error| {
                if json {
                    json_error(
                        "authoring",
                        "write-failed",
                        format!("error writing {:?}: {error}", output),
                    );
                }
                eprintln!("error writing {:?}: {error}", output);
                std::process::exit(1);
            });

            if json {
                #[derive(serde::Serialize)]
                struct CompileOutput {
                    bytes_written: usize,
                    output_path: String,
                    source_map: authoring::AuthoringSourceMap,
                }

                json_success(
                    "authoring",
                    &CompileOutput {
                        bytes_written: bytes.len(),
                        output_path: output.display().to_string(),
                        source_map,
                    },
                );
            } else {
                eprintln!("wrote {} bytes to {:?}", bytes.len(), output);
            }
        }
        AuthoringCommand::Schema { compact } => {
            let schema = authoring::authoring_schema();
            let output = if compact {
                serde_json::to_string(&schema)
            } else {
                serde_json::to_string_pretty(&schema)
            }
            .unwrap_or_else(|error| {
                if global_json {
                    json_error(
                        "authoring",
                        "encode-failed",
                        format!("JSON serialization failed: {error}"),
                    );
                }
                eprintln!("JSON serialization failed: {error}");
                std::process::exit(1);
            });
            println!("{output}");
        }
    }
}

fn exit_lowering_error(error: AuthoringError, json: bool) -> ! {
    if json {
        let envelope = serde_json::json!({
            "ok": false,
            "command": "authoring",
            "code": "lowering-failed",
            "message": "AuthoringSpec lowering failed",
            "diagnostics": error.diagnostics,
        });
        eprintln!("{envelope}");
    } else {
        for diagnostic in error.diagnostics {
            eprintln!(
                "{} [{}]: {}",
                diagnostic.path, diagnostic.code, diagnostic.message
            );
        }
    }
    std::process::exit(1);
}
