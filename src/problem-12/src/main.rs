pub fn int_to_roman(num: i32) -> String {
    let d1 = num % 10;
    let d2 = (num / 10) % 10;
    let d3 = (num / 100) % 10;
    let d4 = (num / 1000) % 10;

    let unit = |n: i32, c1: char, c2: char, c3: char| {
        match n {
            1 => vec![c1],
            2 => vec![c1, c1],
            3 => vec![c1, c1, c1],
            4 => vec![c1, c2],
            5 => vec![c2],
            6 => vec![c2, c1],
            7 => vec![c2, c1, c1],
            8 => vec![c2, c1, c1, c1],
            9 => vec![c1, c3],
            _ => vec![],
        }
    };

    [unit(d1, 'I', 'V', 'X'), unit(d2, 'X', 'L', 'C'), unit(d3, 'C', 'D', 'M'), unit(d4, 'M', '_', '_')]
        .iter().rev().flat_map(|s| s.iter()).collect()
}

#[test]
fn test_int_to_roman() {
    assert_eq!(int_to_roman(4), "IV");
    assert_eq!(int_to_roman(58), "LVIII");
    assert_eq!(int_to_roman(1994), "MCMXCIV")
}