pub fn number_to_words(number: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn test_number_to_words() {
        assert_eq!(number_to_words(0), "zero");
        assert_eq!(number_to_words(1), "one");
        assert_eq!(number_to_words(13), "thirteen");
        assert_eq!(number_to_words(85), "eighty-five");
        assert_eq!(
            number_to_words(5237),
            "five thousand two hundred thirty-seven"
        );
        assert_eq!(
            number_to_words(987654321),
            "nine hundred eighty-seven million six hundred fifty-four thousand three hundred twenty-one"
        );
        assert_eq!(number_to_words(usize::MAX), "eightteen quintillion four hundred fourty-six quadrillion seven hundred fourty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen");
    }
}
