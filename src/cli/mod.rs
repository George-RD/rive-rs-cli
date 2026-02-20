use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rive-cli",
    about = "Generate Rive .riv files programmatically",
    arg_required_else_help = true
)]
pub struct Cli {
    #[arg(long, help = "List available artboard size presets")]
    pub list_presets: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
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
