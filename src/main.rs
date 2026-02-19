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
            eprintln!("generate: not yet implemented");
            eprintln!("  input: {:?}", input);
            eprintln!("  output: {:?}", output);
            eprintln!("  file_id: {}", file_id);
            std::process::exit(1);
        }
        cli::Command::Validate { file } => {
            eprintln!("validate: not yet implemented");
            eprintln!("  file: {:?}", file);
            std::process::exit(1);
        }
        cli::Command::Inspect { file, json } => {
            eprintln!("inspect: not yet implemented");
            eprintln!("  file: {:?}", file);
            eprintln!("  json: {}", json);
            std::process::exit(1);
        }
    }
}
