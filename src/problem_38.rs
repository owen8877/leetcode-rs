pub fn count_and_say(n: i32) -> String {
    let mut result = "1".to_string();
    for _ in 1..n {
        let mut vec = Vec::<(usize, char)>::new();
        let c: Vec<char> = result.chars().collect();
        let mut ref_pos = 0;
        let mut curr_pos = 0;
        loop {
            if curr_pos >= c.len() {
                vec.push((curr_pos-ref_pos, c[ref_pos]));
                break
            }
            if c[curr_pos] != c[ref_pos] {
                vec.push((curr_pos-ref_pos, c[ref_pos]));
                ref_pos = curr_pos;
            }
            curr_pos += 1;
        }

        result = vec.iter().map(|(n, x)| format!("{}{}", n, x)).fold("".to_string(), |acc, x| acc + x.as_str())
    }
    result
}

#[test]
fn test_count_and_say() {
    assert_eq!(count_and_say(1), "1".to_string());
    assert_eq!(count_and_say(4), "1211".to_string());
    assert_eq!(count_and_say(5), "111221".to_string());
}