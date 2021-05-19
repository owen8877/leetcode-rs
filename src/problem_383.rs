pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    fn freq(s: String) -> [usize; 26] {
        let mut f = [0; 26];
        for c in s.chars() {
            f[(c as u8 - 'a' as u8) as usize] += 1;
        }
        f
    }

    let r = freq(ransom_note);
    let m = freq(magazine);
    for i in 0..26 {
        if r[i] > m[i] {
            return false
        }
    }
    true
}