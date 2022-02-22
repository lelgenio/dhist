use clap::{
    Command,
    Arg,
};

pub fn get_cli() -> clap::Command<'static> {
    Command::new("dhist")
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .arg_required_else_help(true)
        // .arg(
        //     Arg::new("histfile-path")
        //         .short('H')
        //         .long("history")
        //         .about("Path to history file"),
        // )
        .subcommand(Command::new(Mode::Query).about("Print history"))
        .subcommand(
            Command::new(Mode::Sort).about("Sort input by history frequency"),
        )
        .subcommand(
            Command::new(Mode::Increment).about("Increase usage of input by 1"),
        )
        .subcommand(
            Command::new(Mode::Wrap)
                .about("Wrap a command to sort before and increment after")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("command")
                        .required(true)
                        .multiple_values(true)
                        .last(true),
                ),
        )
}

pub enum Mode {
    Query,
    Sort,
    Increment,
    Wrap,
}

impl From<Mode> for String {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Query => "query".to_string(),
            Mode::Sort => "sort".to_string(),
            Mode::Increment => "increment".to_string(),
            Mode::Wrap => "wrap".to_string(),
        }
    }
}

impl From<&str> for Mode {
    fn from(val: &str) -> Self {
        match val {
            "query" => Mode::Query,
            "sort" => Mode::Sort,
            "increment" => Mode::Increment,
            "wrap" => Mode::Wrap,
            _ => unreachable!(),
        }
    }
}
