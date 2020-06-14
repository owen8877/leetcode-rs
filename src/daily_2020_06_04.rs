pub fn reverse_string(s: &mut Vec<char>) {
    if s.len() <= 1 {
        return
    }

    for i in 0..s.len()/2 {
        let j = s.len()-i-1;
        let c = s[i];
        s[i] = s[j];
        s[j] = c;
    }
}

#[test]
fn test_reverse_string() {
    let mut a = vec!['a', 'b', 'c'];
    reverse_string(&mut a);
    assert_eq!(a, vec!['c', 'b', 'a']);

    a = vec!['a', 'b', 'c', 'd'];
    reverse_string(&mut a);
    assert_eq!(a, vec!['d', 'c', 'b', 'a']);
}