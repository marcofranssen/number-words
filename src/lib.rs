#[cfg(test)]
mod tests;

pub trait Language {
    fn number_to_words(&self, number: usize) -> String;
}

pub struct English;

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
