#![allow(dead_code, unused_imports)]

mod builder;
mod cli;
mod encoder;
mod objects;
mod validator;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
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
            let scene = builder::build_scene(&spec);
            let refs: Vec<&dyn objects::core::RiveObject> = scene.iter().map(|o| &**o).collect();
            let bytes = encoder::encode_riv(&refs, file_id);
            std::fs::write(&output, &bytes).unwrap_or_else(|e| {
                eprintln!("error writing {:?}: {}", output, e);
                std::process::exit(1);
            });
            eprintln!("wrote {} bytes to {:?}", bytes.len(), output);
        }
        cli::Command::Validate { file } => {
            eprintln!("validate: not yet implemented");
            eprintln!("  file: {:?}", file);
        }
        cli::Command::Inspect { file, json } => {
            eprintln!("inspect: not yet implemented");
            eprintln!("  file: {:?}", file);
            eprintln!("  json: {}", json);
        }
    }
}
