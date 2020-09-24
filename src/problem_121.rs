pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let n = prices.len();
    if n == 0 {
        return 0
    }
    let mut max_price = prices[n-1];
    for i in (0..n-1).rev() {
        profit = std::cmp::max(profit, max_price - prices[i]);
        max_price = std::cmp::max(max_price, prices[i]);
    }
    profit
}