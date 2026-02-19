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
        #[arg(help = "Path to the JSON scene input")]
        input: PathBuf,
        #[arg(short, long, default_value = "output.riv", help = "Output .riv path")]
        output: PathBuf,
        #[arg(long, default_value = "0", help = "Rive file id written in header")]
        file_id: u64,
    },
    Validate {
        #[arg(help = "Path to .riv file to validate")]
        file: PathBuf,
    },
    Inspect {
        #[arg(help = "Path to .riv file to inspect")]
        file: PathBuf,
        #[arg(long, help = "Output parsed result as JSON")]
        json: bool,
    },
}
