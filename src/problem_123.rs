pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = prices.len();
    if n < 2 {
        return 0
    }

    let mut max_price = prices[n-1];
    let mut profit = 0;
    let mut two_profit = 0;
    let mut one_day_max_last = 0;
    let mut max_comb = prices[n-2] + one_day_max_last;
    for j in (0..n-1).rev() {
        profit = max(profit, max_price - prices[j]);
        max_price = max(max_price, prices[j]);
        two_profit = max(two_profit, max_comb - prices[j]);
        max_comb = max(max_comb, prices[j] + one_day_max_last);
        two_profit = max(two_profit, prices[n-1] - prices[j]);
        one_day_max_last = profit;
    }

    two_profit
}