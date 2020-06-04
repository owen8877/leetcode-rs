pub fn length_of_longest_substring(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    if n <= 1 {
        return n as i32;
    }

    let mut location_arr: Vec<usize> = vec![0; n];
    let mut next_location_table: Vec<i32> = vec![-1; 256];

    for i in (0..n).rev() {
        let char = s[i] as usize;
        match next_location_table[char] {
            -1 => {
                // This char is first seen.
                location_arr[i] = i;
                next_location_table[char] = i as i32;
            },
            k => {
                // This char has been seen before.
                location_arr[i] = k as usize;
                next_location_table[char] = i as i32;
            },
        }
    }

    let mut longest = 0;
    for end in (1..n).rev() {
        for start in (0..end).rev() {
            if location_arr[start] <= end && location_arr[start] > start {
                longest = std::cmp::max(longest, end-start);
                break;
            }
            longest = std::cmp::max(longest, end-start+1);
        }
    }

    longest as i32
}

#[test]
pub fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("au")), 2);
}