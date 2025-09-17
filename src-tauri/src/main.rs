// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Cli(cli::Args),
    Gui,
    Tray,
    Service,
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Gui) {
        Commands::Cli(args) => cli::main(args),
        Commands::Gui => desktop_lib::run(true),
        Commands::Tray => desktop_lib::run(false),
        Commands::Service => service::main(),
    }
}
