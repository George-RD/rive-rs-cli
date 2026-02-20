#![allow(dead_code, unused_imports)]

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
        cli::Command::Inspect { file, json } => {
            let bytes = std::fs::read(&file).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {}", file, e);
                std::process::exit(1);
            });
            if json {
                match validator::parse_riv(&bytes) {
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
                match validator::inspect_riv(&bytes) {
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
    }
}
