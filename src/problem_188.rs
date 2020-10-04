pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    use std::cmp::*;
    let n = prices.len();
    if n < 2 {
        return 0
    }

    let k = min(k as usize, n/2+2);
    let mut m = vec![0; k+1];
    let mut rsm = vec![0; k];
    let mut rm = vec![0; k];
    for i in (0..n-1).rev() {
        for j in 0..k {
            let rsm_ij = max(rsm[j], m[j+1] + prices[i+1]);
            rm[j] = max(rm[j], m[j]);
            rsm[j] = rsm_ij;
            m[j] = max(rsm[j]-prices[i], rm[j]);
        }
    }
    m[0] as i32
}