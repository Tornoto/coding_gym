/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0 {
        return 0;
    }

    let n = prices.len();
    // 假设初始拥有的现金数是 0
    // dp[i][0] 表示第 i 天持有股票拥有的现金 第一天是 -prices[0]
    // dp[i][1] 表示第 i 天不持有股票拥有的现金 第一天是 0
    let mut dp = vec![(0, 0); n];
    dp[0].0 = -prices[0];
    dp[0].1 = 0;

    for i in 1..n {
        dp[i].0 = dp[i - 1].0.max(-prices[i]);
        dp[i].1 = dp[i - 1].1.max(prices[i] + dp[i - 1].0);
    }

    dp[n - 1].1
}

/// 上面的 dp 公式发现 i 依赖 i-1 因此可以不用使用数组
pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
    let n = prices.len();

    let mut dp1 = (-prices[0], 0);

    for i in 1..n {
        let mut cur = (0, 0);
        cur.0 = dp1.0.max(-prices[i]);
        cur.1 = dp1.1.max(prices[i] + dp1.0);

        dp1 = cur;
    }

    dp1.1
}
