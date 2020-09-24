pub fn num_distinct(s: String, t: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();

    let ns = s.len();
    let nt = t.len();
    let mut nums = vec![vec![1; nt+1]; ns+1];
    for i in 0..nt {
        nums[ns][i] = 0;
    }
    for j in (0..ns).rev() {
        for i in (0..nt).rev() {
            nums[j][i] = nums[j+1][i];
            if s[j] == t[i] {
                nums[j][i] += nums[j+1][i+1];
            }
        }
        println!("{:?}", nums);
    }

    nums[0][0]
}

#[test]
fn test_num_distinct() {
    assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
}