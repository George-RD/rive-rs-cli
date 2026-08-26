mod ai;
mod authoring;
mod catalog;
mod output;
mod scene;
mod visual;

use clap::Parser;
use rive_cli::builder;
#[cfg(feature = "mcp")]
use rive_cli::mcp;

use crate::cli::{Cli, Command};
use output::json_error;

pub fn run() {
    let command_line = std::env::args().collect::<Vec<_>>();
    let json_requested = command_line.iter().any(|argument| argument == "--json");
    let json_command = command_line
        .iter()
        .find_map(|argument| match argument.as_str() {
            "generate" | "new" | "validate" | "inspect" | "decompile" | "render" | "compare"
            | "schema" | "types" | "describe" | "authoring" | "ai" => Some(argument.as_str()),
            _ => None,
        })
        .unwrap_or("cli");
    let cli = Cli::try_parse().unwrap_or_else(|error| {
        if json_requested
            && !matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            )
        {
            json_error(json_command, "usage", error);
        }
        error.exit();
    });

    if cli.list_presets {
        if cli.json {
            let json = serde_json::to_string_pretty(&builder::artboard_presets()).unwrap_or_else(
                |error| {
                    eprintln!("JSON serialization failed: {}", error);
                    std::process::exit(1);
                },
            );
            println!("{}", json);
        } else {
            for preset in builder::artboard_presets() {
                println!("{}: {}x{}", preset.name, preset.width, preset.height);
            }
        }
        return;
    }

    #[cfg(feature = "mcp")]
    if cli.mcp {
        mcp::run_server();
        return;
    }

    let command = cli.command.unwrap_or_else(|| {
        if cli.json {
            json_error("cli", "usage", "no command provided");
        }
        eprintln!("no command provided");
        std::process::exit(1);
    });

    let global_json = cli.json;
    match command {
        command @ (Command::Generate { .. }
        | Command::New { .. }
        | Command::Validate { .. }
        | Command::Inspect { .. }
        | Command::Decompile { .. }) => scene::run(command, global_json),
        command @ (Command::Render { .. } | Command::Compare { .. }) => {
            visual::run(command, global_json)
        }
        command @ (Command::Schema { .. } | Command::Types { .. } | Command::Describe { .. }) => {
            catalog::run(command, global_json)
        }
        command @ Command::Authoring { .. } => authoring::run(command, global_json),
        command @ Command::Ai { .. } => ai::run(command),
    }
}
