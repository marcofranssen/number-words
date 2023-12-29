use std::usize;

#[cfg(test)]
mod tests;

pub trait Language {
    fn number_to_words(&self, number: usize) -> String;
}

pub struct English;
pub struct Dutch;

impl Language for English {
    fn number_to_words(&self, number: usize) -> String {
        let units = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let teens = [
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
        let tens = [
            "", "", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty", "ninety",
        ];
        let scales = [
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
                    format!(
                        "{} {} {}",
                        units[hundred],
                        scales[0],
                        self.number_to_words(rest)
                    )
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
                        let mut thousand_words = self.number_to_words(thousand);
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
}

// https://nl.wikipedia.org/wiki/Korte_en_lange_schaal
impl Language for Dutch {
    fn number_to_words(&self, number: usize) -> String {
        let units = [
            "nul", "een", "twee", "drie", "vier", "vijf", "zes", "zeven", "acht", "negen",
        ];
        let teens = [
            "tien",
            "elf",
            "twaalf",
            "dertien",
            "veertien",
            "vijftien",
            "zestien",
            "zeventien",
            "achttien",
            "negentien",
        ];
        let tens = [
            "", "", "twintig", "dertig", "veertig", "vijftig", "zestig", "zeventig", "tachtig",
            "negentig",
        ];
        let scales = [
            "honderd", "duizend", "miljoen", "miljard", "biljoen", "biljard", "triljoen",
        ];
        match number {
            0..=9 => units[number].to_string(),
            10..=19 => teens[number - 10].to_string(),
            20..=99 => {
                let ten = tens[number / 10];
                let unit = number % 10;
                if unit == 0 {
                    ten.to_string()
                } else if units[unit].ends_with('e') {
                    format!("{}ën{}", units[unit], ten)
                } else {
                    format!("{}en{}", units[unit], ten)
                }
            }
            100..=199 => {
                let rest = number % 100;
                if rest == 0 {
                    scales[0].to_string()
                } else {
                    format!("{}{}", scales[0], self.number_to_words(rest))
                }
            }
            200..=999 => {
                let hundred = number / 100;
                let rest = number % 100;
                if rest == 0 {
                    format!("{} {}", units[hundred], scales[0])
                } else {
                    format!(
                        "{}{}{}",
                        units[hundred],
                        scales[0],
                        self.number_to_words(rest)
                    )
                }
            }
            1000000..=999999999 => {
                let million = number / 1000000;
                let rest = number % 1000000;
                if rest == 0 {
                    format!("{} {}", self.number_to_words(million), scales[2])
                } else {
                    format!(
                        "{} {} {}",
                        self.number_to_words(million),
                        scales[2],
                        self.number_to_words(rest)
                    )
                }
            }
            1000000000..=999999999999 => {
                let billion = number / 1000000000;
                let rest = number % 1000000000;
                if rest == 0 {
                    format!("{} {}", self.number_to_words(billion), scales[3])
                } else {
                    format!(
                        "{} {} {}",
                        self.number_to_words(billion),
                        scales[3],
                        self.number_to_words(rest)
                    )
                }
            }
            1000000000000..=999999999999999 => {
                let trillion = number / 1000000000000;
                let rest = number % 1000000000000;
                if rest == 0 {
                    format!("{} {}", self.number_to_words(trillion), scales[4])
                } else {
                    format!(
                        "{} {} {}",
                        self.number_to_words(trillion),
                        scales[4],
                        self.number_to_words(rest)
                    )
                }
            }
            1000000000000000..=999999999999999999 => {
                let quadrillion = number / 1000000000000000;
                let rest = number % 1000000000000000;
                if rest == 0 {
                    format!("{} {}", self.number_to_words(quadrillion), scales[5])
                } else {
                    format!(
                        "{} {} {}",
                        self.number_to_words(quadrillion),
                        scales[5],
                        self.number_to_words(rest)
                    )
                }
            }
            1000000000000000000..=usize::MAX => {
                let quadrillion = number / 1000000000000000;
                let rest = number % 1000000000000000;
                if rest == 0 {
                    format!("{} {}", self.number_to_words(quadrillion), scales[5])
                } else {
                    format!(
                        "{} {} {}",
                        self.number_to_words(quadrillion),
                        scales[5],
                        self.number_to_words(rest)
                    )
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
                        let mut thousand_words = self.number_to_words(thousand);
                        if scale_index > 0 {
                            thousand_words = format!("{}{}", thousand_words, scales[scale_index]);
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
}
