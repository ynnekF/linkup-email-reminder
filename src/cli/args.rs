// Shared argument definitions and utilities
// You can add common argument builders here as your CLI grows

use clap::Arg;

pub fn verbose_arg() -> Arg<'static> {
    Arg::new("verbose")
        .long("verbose")
        .short('v')
        .action(clap::ArgAction::SetTrue)
        .help("Use verbose output")
}

pub fn config_arg() -> Arg<'static> {
    Arg::new("config")
        .long("config")
        .value_name("FILE")
        .help("Use custom config file")
}
