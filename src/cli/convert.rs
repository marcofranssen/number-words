use clap::{value_parser, Arg, ArgMatches, Command};

use number2words::{English, Language};

pub fn new() -> Command {
    Command::new("convert")
        .about("Converts a number to words")
        .arg(
            Arg::new("number")
                .required(true)
                .index(1)
                .value_parser(value_parser!(usize)),
        )
}

pub fn run(m: &ArgMatches) {
    let number: usize = *m.get_one("number").expect("number is required");
    let number_in_words = English.number_to_words(number);
    println!("{}", number_in_words);
}
