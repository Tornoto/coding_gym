/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
/// 动态规划解法
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }

    let n = prices.len();
    // 仍然定义 dp[i] 为 持有股票和不持有股票拥有的最大现金量
    let mut dp = vec![(0, 0); n];
    // dp[0].0 持有为 -prices[0]
    // dp[0].1 不持有为 0
    dp[0] = (-prices[0], 0);

    for i in 1..n {
        // 由于可以多次买入 可以累计之前股票出售带来的收益 dp[i-1].1
        dp[i].0 = dp[i - 1].0.max(dp[i - 1].1 - prices[i]);
        dp[i].1 = dp[i - 1].1.max(dp[i - 1].0 + prices[i]);
    }

    dp[n - 1].1
}
