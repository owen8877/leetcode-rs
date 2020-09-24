pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s.chars().collect();

    fn core(s: &[char]) -> bool {
        fn is_alphabet(c: char) -> bool {
            (c as u8 >= 'a' as u8 && c as u8 <= 'z' as u8) || (c as u8 >= 'A' as u8 && c as u8 <= 'Z' as u8) || (c as u8 >= '0' as u8 && c as u8 <= '9' as u8)
        }

        fn compare(c1: char, c2: char) -> bool {
            (c1 == c2) || (c1 as u8 == c2 as u8 + 32u8 && c2 as u8 >= 'A' as u8) || (c1 as u8 + 32u8 == c2 as u8 && c1 as u8 >= 'A' as u8)
        }

        let n = s.len();

        n == 0 || (0..n).filter(|&i| is_alphabet(s[i])).next().map_or(true, |f_index| (0..n).rev().filter(|&i| is_alphabet(s[i])).next().map_or(true, |b_index| (f_index == b_index) || compare(s[f_index], s[b_index]) && core(&s[f_index+1..b_index])))
    }

    core(s.as_slice())
}