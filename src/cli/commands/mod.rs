use crate::core::{errors::CliResult, Context};
use clap::{ArgMatches, Command};

// Function signature for command executors
pub type ExecFn = fn(&mut Context, &ArgMatches) -> CliResult;

// Return all builtin commands for clap
pub fn builtin() -> Vec<Command<'static>> {
    vec![
        send::cli(),
        list::cli(),
        // add::cli(),
        // remove::cli(),
        // status::cli(),
        // next::cli(),
        // config::cli(),
        // template::cli(),
        // help::cli(),
    ]
}

// Map command names to their executor functions
pub fn builtin_exec(cmd: &str) -> Option<ExecFn> {
    match cmd {
        "send" => Some(send::exec),
        "list" => Some(list::exec),
        // "add" => Some(add::exec),
        // "remove" => Some(remove::exec),
        // "status" => Some(status::exec),
        // "next" => Some(next::exec),
        // "config" => Some(config::exec),
        // "template" => Some(template::exec),
        // "help" => Some(help::exec),
        _ => None,
    }
}

// Declare all command modules
pub mod list;
pub mod send;
// pub mod add;
// pub mod remove;
// pub mod status;
// pub mod next;
// pub mod config;
// pub mod template;
// pub mod help;
