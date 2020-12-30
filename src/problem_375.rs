pub fn get_money_amount(n: i32) -> i32 {
    use std::cmp::*;

    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for offset in 1..=n {
        for a in 1..=n - offset {
            let b = a + offset;
            dp[a][b] = (a..b).map(|k| max(dp[a][k - 1], dp[k + 1][b]) + k).fold(1 << 30, |acc, x| min(acc, x))
        }
    }

    dp[1][n] as i32
}