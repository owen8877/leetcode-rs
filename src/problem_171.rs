pub fn title_to_number(s: String) -> i32 {
    s.chars().fold(0, |acc, x| acc * 26 + (x as u8 - 'A' as u8 + 1) as i32)
}