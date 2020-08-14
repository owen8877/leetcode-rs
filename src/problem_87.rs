pub fn is_scramble(s1: String, s2: String) -> bool {
    fn core(s1: &[char], s2: &[char]) -> bool {
        use std::collections::HashMap;
        if s1 == s2 {
            return true
        }
        
        /*fn get_freq(s: &[char]) -> HashMap::<char, usize> {
            let mut map = HashMap::new();
            for c in s {
                map.entry(*c).and_modify(|x| *x += 1).or_insert(1);
            }
            map
        }*/
        
        fn get_freq(s: &[char]) -> Vec::<usize> {
            let mut vec = vec![0; 26];
            for c in s {
                vec[(*c as u8 - 'a' as u8) as usize] += 1;
            }
            vec
        }
        
        if get_freq(s1) != get_freq(s2) {
            return false
        }
        
        for i in 1..s1.len() {
            if core(&s1[..i], &s2[..i]) && core(&s1[i..], &s2[i..]) {
                return true
            }
            if core(&s1[..i], &s2[s1.len()-i..]) && core(&s1[i..], &s2[..s1.len()-i]) {
                return true
            }
        }
        false
    }
    
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    core(s1.as_slice(), s2.as_slice())
}
