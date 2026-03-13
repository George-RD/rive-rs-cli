use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rive-cli",
    version,
    about = "Generate Rive .riv files programmatically",
    arg_required_else_help = true
)]
pub struct Cli {
    #[arg(long, help = "List available artboard size presets")]
    pub list_presets: bool,

    #[arg(long, help = "Output as JSON")]
    pub json: bool,

    #[cfg(feature = "mcp")]
    #[arg(long, help = "Run as MCP server over stdio")]
    pub mcp: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(
        about = "Generate a .riv file from a JSON scene spec",
        long_about = "Generate a .riv file from a JSON scene spec.\n\nExamples:\n  rive-cli generate scene.json\n  rive-cli generate scene.json -o my_animation.riv\n  rive-cli generate scene.json --file-id 42"
    )]
    Generate {
        #[arg(help = "Path to the JSON scene input")]
        input: PathBuf,
        #[arg(short, long, default_value = "output.riv", help = "Output .riv path")]
        output: PathBuf,
        #[arg(long, default_value = "0", help = "Rive file id written in header")]
        file_id: u64,
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Validate a .riv file for structural correctness")]
    Validate {
        #[arg(help = "Path to .riv file to validate")]
        file: PathBuf,
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    #[command(
        about = "Inspect objects and properties in a .riv file",
        long_about = "Inspect objects and properties in a .riv file.\n\nExamples:\n  rive-cli inspect output.riv\n  rive-cli inspect output.riv --json\n  rive-cli inspect output.riv --type-name Shape\n  rive-cli inspect output.riv --artboard-name Main --local-index 1"
    )]
    Inspect {
        #[arg(help = "Path to .riv file to inspect")]
        file: PathBuf,
        #[arg(long, help = "Output parsed result as JSON")]
        json: bool,
        #[arg(
            long = "artboard-index",
            value_name = "N",
            help = "Filter by artboard index"
        )]
        artboard_index: Vec<usize>,
        #[arg(
            long = "artboard-name",
            value_name = "NAME",
            help = "Filter by artboard name (case-insensitive)"
        )]
        artboard_name: Vec<String>,
        #[arg(
            long = "local-index",
            value_name = "N",
            help = "Filter by artboard-local object index (0 is the artboard itself)"
        )]
        local_index: Vec<usize>,
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
    #[command(about = "Decompile a .riv file to structured JSON")]
    Decompile {
        #[arg(help = "Path to .riv file to decompile")]
        file: PathBuf,
    },
    #[command(about = "AI-assisted .riv generation and evaluation")]
    Ai {
        #[command(subcommand)]
        command: AiCommand,
    },
}

#[derive(Subcommand)]
pub enum AiCommand {
    #[command(
        about = "Generate a .riv file from a natural language prompt or template",
        long_about = "Generate a .riv file from a natural language prompt or template.\n\nExamples:\n  rive-cli ai generate --template bounce\n  rive-cli ai generate --prompt \"a spinning logo\" -o logo.riv\n  rive-cli ai generate --prompt \"pulsing button\" --max-retries 5"
    )]
    Generate {
        #[arg(
            long,
            group = "input",
            help = "Natural language prompt describing the animation"
        )]
        prompt: Option<String>,
        #[arg(
            long,
            group = "input",
            help = "Use a built-in template (bounce, spinner, pulse, fade)"
        )]
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
    #[command(about = "Run evaluation suites for AI-generated animations")]
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
