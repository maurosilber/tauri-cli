// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{CommandFactory, Parser, Subcommand};

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
    let Some(command) = parse() else {
        let _ = Cli::command().print_long_help();
        return;
    };

    match command {
        Commands::Cli(args) => cli::main(args),
        Commands::Gui => desktop_lib::run(true),
        Commands::Tray => desktop_lib::run(false),
        Commands::Service => service::main(),
    }
}

#[cfg(target_os = "windows")]
fn parse() -> Option<Commands> {
    use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};

    // Attach parent process before clap parsing,
    // since it will write to the console if the arguments are not valid.
    let invoked_from_console = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) } != 0;
    let cli = Cli::parse();

    match cli.command {
        Some(c) => Some(c),
        None if invoked_from_console => None,
        None => Some(Commands::Gui),
    }
}

#[cfg(unix)]
fn parse() -> Option<Commands> {
    Cli::parse().command
}
