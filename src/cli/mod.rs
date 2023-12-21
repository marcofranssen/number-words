mod app;
mod convert;

use clap::{ArgMatches, Command};

pub fn new() -> Command {
    app::new().subcommand(convert::new())
}

pub fn run_convert(m: &ArgMatches) {
    convert::run(m)
}
