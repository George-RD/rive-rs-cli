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
        #[arg(
            long = "type-key",
            value_name = "N",
            help = "Filter by object type key"
        )]
        type_key: Vec<u16>,
        #[arg(
            long = "type-name",
            value_name = "NAME",
            help = "Filter by object type name (case-insensitive)"
        )]
        type_name: Vec<String>,
        #[arg(
            long = "object-index",
            value_name = "N",
            help = "Filter by global object index"
        )]
        object_index: Vec<usize>,
        #[arg(
            long = "property-key",
            value_name = "N",
            help = "Filter displayed properties by key"
        )]
        property_key: Vec<u16>,
    },
    Ai {
        #[command(subcommand)]
        command: AiCommand,
    },
}

#[derive(Subcommand)]
pub enum AiCommand {
    Generate {
        #[arg(long, help = "Natural language prompt describing the animation")]
        prompt: Option<String>,
        #[arg(long, help = "Use a built-in template (bounce, spinner, pulse, fade)")]
        template: Option<String>,
        #[arg(short, long, default_value = "output.riv", help = "Output .riv path")]
        output: std::path::PathBuf,
        #[arg(long, default_value = "0", help = "Rive file id")]
        file_id: u64,
        #[arg(long, help = "Output SceneSpec JSON without encoding to .riv")]
        dry_run: bool,
        #[arg(long, help = "Override AI model (e.g. gpt-4o, gpt-4.1)")]
        model: Option<String>,
        #[arg(long, help = "AI provider (template, openai)")]
        provider: Option<String>,
        #[arg(
            long,
            default_value = "3",
            help = "Max auto-repair retries (0 = no repair)"
        )]
        max_retries: u8,
    },
    Lab {
        #[arg(long, help = "Path to eval suite JSON file")]
        suite: PathBuf,
        #[arg(
            long,
            default_value = "evals/runs",
            help = "Output directory for eval runs"
        )]
        output_dir: PathBuf,
        #[arg(long, default_value = "0", help = "Rive file id")]
        file_id: u64,
        #[arg(
            long,
            default_value = "3",
            help = "Max auto-repair retries (0 = no repair)"
        )]
        max_retries: u8,
        #[arg(long, help = "Optional baseline JSON file for drift detection")]
        baseline: Option<PathBuf>,
        #[arg(long, help = "Write baseline JSON from this run")]
        write_baseline: Option<PathBuf>,
    },
}
