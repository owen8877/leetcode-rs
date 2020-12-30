pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut comb = vec![0; target as usize + 1];
    comb[0] = 1;
    for i in 1..=target as usize {
        for &j in &nums {
            if j <= i as i32 {
                comb[i] += comb[i - j as usize];
            }
        }
    }
    comb[target as usize]
}