use crate::{
    cli::args::{config_arg, verbose_arg},
    core::{errors::CliResult, Context},
};
use clap::Command;

mod args;
mod commands;

pub fn main(context: &mut Context) -> CliResult {
    // Build the main CLI command with all subcommands
    let app = cli(context);

    // Parse command line arguments
    let matches = app.get_matches();

    // Extract global arguments first
    if matches.get_flag("verbose") {
        context.set_verbose(true);
    }

    // Dispatch to appropriate subcommand
    match matches.subcommand() {
        Some((cmd, sub_matches)) => {
            if let Some(exec_fn) = commands::builtin_exec(cmd) {
                exec_fn(context, sub_matches)
            } else {
                Err(format!("Unknown command: {}", cmd).into())
            }
        }
        None => {
            // No subcommand provided - show help or run default action
            let mut app = cli(context);
            app.print_help()?;
            Ok(())
        }
    }
}

fn cli(_context: &Context) -> Command<'static> {
    Command::new("linkup-email-reminder")
        .version("1.0.0")
        .about("Email reminder system for coordinating weekly group activities")
        .long_about(
            "A CLI tool for managing turn-based email reminders with persistent state management.",
        )
        .arg(verbose_arg().global(true))
        .arg(config_arg().global(true))
        // Add all builtin subcommands
        .subcommands(commands::builtin())
        // Allow external subcommands (for extensibility)
        .allow_external_subcommands(true)
        // Custom help template (optional, like cargo)
        .help_template(
            "{name} {version}\n\
             {about}\n\n\
             USAGE:\n    {usage}\n\n\
             OPTIONS:\n{options}\n\n\
             COMMANDS:\n{subcommands}\n\n\
             See '{name} help <command>' for more information on a specific command.",
        )
}
