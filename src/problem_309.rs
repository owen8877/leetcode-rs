pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = prices.len();

    if n <= 1 {
        return 0;
    }

    let mut profit = vec![0; n];

    for j in (0..n - 1).rev() {
        let mut p = profit[j + 1];
        for i in j + 1..n - 2 {
            p = max(p, prices[i] - prices[j] + profit[i + 2]);
        }
        for i in n - 2..n {
            p = max(p, prices[i] - prices[j]);
        }
        profit[j] = p;
    }

    profit[0]
}