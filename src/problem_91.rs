pub fn num_decodings(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>().into_iter().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>();

    fn count_ways(s: &[u8], index: usize, cache: &mut Vec<Option<i32>>) -> i32 {
        if let Some(c) = cache[index] {
            return c
        }
        let c = {
            let i = s[index];
            if i == 0 {
                0
            } else if i == 1 {
                count_ways(s, index+1, cache) + if s.len()-index > 1 { count_ways(s, index+2, cache) } else { 0 }
            } else if i == 2 {
                count_ways(s, index+1, cache) + if s.len()-index > 1 && s[index+1] <= 6 { count_ways(s, index+2, cache) } else { 0 }
            } else {
                count_ways(s, index+1, cache)
            }
        };
        cache[index] = Some(c);
        c
    }

    let n = s.len();
    let mut cache = vec![None; n+1];
    cache[n] = Some(1);
    count_ways(s.as_slice(), 0, &mut cache)
}
