pub fn max_product(words: Vec<String>) -> i32 {
    let mut nums = vec![0; words.len()];
    let mut lens = vec![0; words.len()];
    for (i, word) in words.iter().enumerate() {
        lens[i] = word.len();
        for c in word.chars() {
            nums[i] |= 1 << (c as u8 - 'a' as u8);
        }
    }

    let mut m = 0;
    for i in 0..words.len() {
        for j in i + 1..words.len() {
            if nums[i] & nums[j] == 0 {
                m = std::cmp::max(m, lens[i] * lens[j]);
            }
        }
    }
    m as i32
}