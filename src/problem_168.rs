pub fn convert_to_title(mut n: i32) -> String {
    let mut result = vec![];
    while n > 0 {
        result.push(('A' as u8 + ((n-1) % 26) as u8) as char);
        n = (n-1) / 26;
    }
    result.iter().rev().collect()
}