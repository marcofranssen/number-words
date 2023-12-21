use clap::Command;

use crate::crate_info::{crate_authors, crate_description, crate_name, crate_version}; // Import the convert_command function from the super module

pub fn new() -> Command {
    Command::new(crate_name())
        .version(crate_version())
        .author(crate_authors())
        .about(crate_description())
}
