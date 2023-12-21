use clap::{value_parser, Arg, ArgMatches, Command};

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
    let number_in_words = number_to_words(number);
    println!("{}", number_in_words);
}

fn number_to_words(number: usize) -> String {
    let units = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens: [&str; 10] = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eightteen",
        "nineteen",
    ];
    let tens: [&str; 10] = [
        "", "", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let scales: [&str; 7] = [
        "hundred",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    match number {
        0..=9 => units[number].to_string(),
        10..=19 => teens[number - 10].to_string(),
        20..=99 => {
            let ten = tens[number / 10];
            let unit = number % 10;
            if unit == 0 {
                ten.to_string()
            } else {
                format!("{}-{}", ten, units[unit])
            }
        }
        100..=999 => {
            let hundred = number / 100;
            let rest = number % 100;
            if rest == 0 {
                format!("{} {}", units[hundred], scales[0])
            } else {
                format!("{} {} {}", units[hundred], scales[0], number_to_words(rest))
            }
        }
        _ => {
            let mut scale_index = 0;
            let mut rest = number;
            let mut words = vec![];
            while rest > 0 {
                let thousand = rest % 1000;
                rest /= 1000;
                if thousand > 0 {
                    let mut thousand_words = number_to_words(thousand);
                    if scale_index > 0 {
                        thousand_words = format!("{} {}", thousand_words, scales[scale_index]);
                    }
                    words.push(thousand_words);
                }
                scale_index += 1;
            }
            words.reverse();
            words.join(" ")
        }
    }
}
