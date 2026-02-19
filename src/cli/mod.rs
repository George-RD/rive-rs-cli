use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rive-cli", about = "Generate Rive .riv files programmatically")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Generate {
        input: PathBuf,
        #[arg(short, long, default_value = "output.riv")]
        output: PathBuf,
        #[arg(long, default_value = "0")]
        file_id: u64,
    },
    Validate {
        file: PathBuf,
    },
    Inspect {
        file: PathBuf,
        #[arg(long)]
        json: bool,
    },
}
