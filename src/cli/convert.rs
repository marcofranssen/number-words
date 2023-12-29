use clap::{value_parser, Arg, ArgMatches, Command};

use number2words::{Dutch, English, Language};

pub fn new() -> Command {
    Command::new("convert")
        .about("Converts a number to words")
        .arg(
            Arg::new("language")
                .long("language")
                .short('l')
                .default_value("english")
                .value_parser(["english", "dutch"]),
        )
        .arg(
            Arg::new("number")
                .required(true)
                .index(1)
                .value_parser(value_parser!(usize)),
        )
}

pub fn run(m: &ArgMatches) {
    let number: usize = *m.get_one("number").expect("number is required");

    let words: String = match m
        .get_one::<String>("language")
        .expect("language is required")
        .as_str()
    {
        "english" => English.number_to_words(number),
        "dutch" => Dutch.number_to_words(number),
        _ => English.number_to_words(number),
    };

    println!("{}", words);
}
