pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let mut dict = HashMap::new();
    dict.insert('I', 1);
    dict.insert('V', 5);
    dict.insert('X', 10);
    dict.insert('L', 50);
    dict.insert('C', 100);
    dict.insert('D', 500);
    dict.insert('M', 1000);

    let mut n = 0;
    let mut last_m = 0;
    for c in s.chars().into_iter().rev() {
        let m = dict.get(&c).unwrap().clone();
        if m >= last_m {
            n += m;
        } else {
            n -= m;
        }
        last_m = m;
    }

    n
}

#[test]
fn test_romain_to_int() {
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
}
