pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let digits = s.chars().map(|c| (c as u8 - '0' as u8) as u64).collect::<Vec<u64>>();

    fn parse_and_insert(digits: &[u64], sums: &mut Vec<u64>, seg_idx: usize, results: &mut Vec<String>) {
        if seg_idx == 4 {
            if digits.len() == 0 {
                results.push(format!("{}.{}.{}.{}", sums[0], sums[1], sums[2], sums[3]))
            }
        } else {
            if digits.len() == 0 {
                return
            }
            sums[seg_idx] = sums[seg_idx] * 10 + digits[0];
            if sums[seg_idx] == 0 {
                parse_and_insert(&digits[1..], sums, seg_idx+1, results);
            } else if sums[seg_idx] < 256 {
                parse_and_insert(&digits[1..], sums, seg_idx, results);
                parse_and_insert(&digits[1..], sums, seg_idx+1, results);
            }
            sums[seg_idx] /= 10;
        }
    }

    let mut results = vec![];
    let mut sums = vec![0; 4];

    parse_and_insert(digits.as_slice(), &mut sums, 0, &mut results);
    results
}

#[test]
fn test_restore_ip_addresses() {
    assert_eq!(restore_ip_addresses("25525511135".to_string()), vec!["255.255.111.35".to_string(), "255.255.11.135".to_string()]);
    assert_eq!(restore_ip_addresses("010010".to_string()), vec!["0.100.1.0".to_string(), "0.10.0.10".to_string()]);
}