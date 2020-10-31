pub fn is_additive_number(num: String) -> bool {
    let num: Vec<u64> = num.chars().map(|c| (c as u8 - '0' as u8) as u64).collect();
    let n = num.len();
    if n < 3 {
        return false;
    }

    fn tonum(s: &[u64]) -> u64 {
        s.iter().fold(0, |acc, &x| acc * 10 + x)
    }

    fn match_arr(mut t: u64, s: &[u64]) -> Option<usize> {
        if t == 0 {
            return if s[0] == 0 {
                Some(1)
            } else { None };
        }
        let mut st = vec![];
        while t > 0 {
            st.push(t % 10);
            t /= 10;
        }
        if st.len() > s.len() {
            return None;
        }
        for pointer in 0..st.len() {
            if st[st.len() - 1 - pointer] != s[pointer] {
                return None;
            }
        }
        Some(st.len())
    }

    for i in 1..n - 1 {
        if num[0] == 0 && i > 1 {
            continue;
        }
        let first_origin = tonum(&num[..i]);
        for j in 1..n - i {
            if num[i] == 0 && j > 1 {
                continue;
            }
            let mut first = first_origin;
            let mut second = tonum(&num[i..i + j]);
            let mut pointer = i + j;
            while pointer < n {
                let sum = first + second;
                match match_arr(sum, &num[pointer..]) {
                    Some(p) => {
                        pointer += p;
                        first = second;
                        second = sum;
                    }
                    None => break
                }
                if pointer == n {
                    return true;
                }
            }
        }
    }
    false
}