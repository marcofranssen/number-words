use super::*;

#[test]
fn test_number_to_words_english() {
    let lang = English;
    assert_eq!(lang.number_to_words(0), "zero");
    assert_eq!(lang.number_to_words(1), "one");
    assert_eq!(lang.number_to_words(13), "thirteen");
    assert_eq!(lang.number_to_words(85), "eighty-five");
    assert_eq!(lang.number_to_words(199), "one hundred ninety-nine");
    assert_eq!(
        lang.number_to_words(5237),
        "five thousand two hundred thirty-seven"
    );
    assert_eq!(
        lang.number_to_words(52_379),
        "fifty-two thousand three hundred seventy-nine"
    );
    assert_eq!(
        lang.number_to_words(345_678),
        "three hundred fourty-five thousand six hundred seventy-eight"
    );
    assert_eq!(
        lang.number_to_words(9_876_543),
        "nine million eight hundred seventy-six thousand five hundred fourty-three"
    );
    assert_eq!(
        lang.number_to_words(98_765_432),
        "ninety-eight million seven hundred sixty-five thousand four hundred thirty-two"
    );
    assert_eq!(
        lang.number_to_words(987_654_321),
        "nine hundred eighty-seven million six hundred fifty-four thousand three hundred twenty-one"
    );
    assert_eq!(
        lang.number_to_words(9_876_543_210),
        "nine billion eight hundred seventy-six million five hundred fourty-three thousand two hundred ten"
    );
    assert_eq!(
        lang.number_to_words(58_765_432_101),
        "fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(158_765_432_101),
        "one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(1_158_765_432_101),
        "one trillion one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(11_158_765_432_101),
        "eleven trillion one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(211_158_765_432_101),
        "two hundred eleven trillion one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(4_211_158_765_432_101),
        "four quadrillion two hundred eleven trillion one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(64_211_158_765_432_101),
        "sixty-four quadrillion two hundred eleven trillion one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(864_211_158_765_432_101),
        "eight hundred sixty-four quadrillion two hundred eleven trillion one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(
        lang.number_to_words(7_864_211_158_765_432_101),
        "seven quintillion eight hundred sixty-four quadrillion two hundred eleven trillion one hundred fifty-eight billion seven hundred sixty-five million four hundred thirty-two thousand one hundred one"
    );
    assert_eq!(lang.number_to_words(usize::MAX), "eightteen quintillion four hundred fourty-six quadrillion seven hundred fourty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen");
}

#[test]
fn test_number_to_words_dutch() {
    let lang = Dutch;
    assert_eq!(lang.number_to_words(0), "nul");
    assert_eq!(lang.number_to_words(1), "een");
    assert_eq!(lang.number_to_words(13), "dertien");
    assert_eq!(lang.number_to_words(85), "vijfentachtig");
    assert_eq!(lang.number_to_words(199), "honderdnegenennegentig");
    assert_eq!(
        lang.number_to_words(5237),
        "vijfduizend tweehonderdzevenendertig"
    );
    assert_eq!(
        lang.number_to_words(52379),
        "tweeënvijftigduizend driehonderdnegenenzeventig"
    );
    assert_eq!(
        lang.number_to_words(345678),
        "driehonderdvijfenveertigduizend zeshonderdachtenzeventig"
    );
    assert_eq!(
        lang.number_to_words(6876542),
        "zes miljoen achthonderdzesenzeventigduizend vijfhonderdtweeënveertig"
    );
    assert_eq!(
        lang.number_to_words(59876543),
        "negenenvijftig miljoen achthonderdzesenzeventigduizend vijfhonderddrieënveertig"
    );
    assert_eq!(
        lang.number_to_words(459876543),
        "vierhonderdnegenenvijftig miljoen achthonderdzesenzeventigduizend vijfhonderddrieënveertig"
    );
    assert_eq!(
        lang.number_to_words(3459876543),
        "drie miljard vierhonderdnegenenvijftig miljoen achthonderdzesenzeventigduizend vijfhonderddrieënveertig"
    );
    assert_eq!(
        lang.number_to_words(23459876542),
        "drieëntwintig miljard vierhonderdnegenenvijftig miljoen achthonderdzesenzeventigduizend vijfhonderdtweeënveertig"
    );
    assert_eq!(
        lang.number_to_words(987654321),
        "negenhonderdzevenentachtig miljoen zeshonderdvierenvijftigduizend driehonderdeenentwintig"
    );
    assert_eq!(
        lang.number_to_words(9876534210),
        "negen miljard achthonderdzesenzeventig miljoen vijfhonderdvierendertigduizend tweehonderdtien"
    );
    assert_eq!(
        lang.number_to_words(98765432101),
        "achtennegentig miljard zevenhonderdvijfenzestig miljoen vierhonderdtweeëndertigduizend honderdeen"
    );
    assert_eq!(
        lang.number_to_words(727654321012),
        "zevenhonderdzevenentwintig miljard zeshonderdvierenvijftig miljoen driehonderdeenentwintigduizend twaalf"
    );
    assert_eq!(
        lang.number_to_words(6727654321012),
        "zes biljoen zevenhonderdzevenentwintig miljard zeshonderdvierenvijftig miljoen driehonderdeenentwintigduizend twaalf"
    );
    assert_eq!(
        lang.number_to_words(56_727_654_321_012),
        "zesenvijftig biljoen zevenhonderdzevenentwintig miljard zeshonderdvierenvijftig miljoen driehonderdeenentwintigduizend twaalf"
    );
    assert_eq!(
        lang.number_to_words(856_727_654_321_012),
        "achthonderdzesenvijftig biljoen zevenhonderdzevenentwintig miljard zeshonderdvierenvijftig miljoen driehonderdeenentwintigduizend twaalf"
    );
    assert_eq!(
        lang.number_to_words(456_727_654_321_012),
        "vierhonderdzesenvijftig biljoen zevenhonderdzevenentwintig miljard zeshonderdvierenvijftig miljoen driehonderdeenentwintigduizend twaalf"
    );
    assert_eq!(
        lang.number_to_words(34_567_276_543_210_129),
        "vierendertig biljard vijfhonderdzevenenzestig biljoen tweehonderdzesenzeventig miljard vijfhonderddrieënveertig miljoen tweehonderdtienduizend honderdnegenentwintig"
    );
    assert_eq!(
        lang.number_to_words(234_567_276_543_210_129),
        "tweehonderdvierendertig biljard vijfhonderdzevenenzestig biljoen tweehonderdzesenzeventig miljard vijfhonderddrieënveertig miljoen tweehonderdtienduizend honderdnegenentwintig"
    );
}
