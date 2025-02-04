/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/description/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n == 0 {
        return 0;
    }

    // 使用动态规划，从前向后计算最大利润
    let mut dp1 = vec![0; n];
    let mut min_price = prices[0];
    for i in 1..n {
        min_price = min_price.min(prices[i]);
        dp1[i] = dp1[i - 1].max(prices[i] - min_price);
    }

    // 使用动态规划，从后向前计算最大利润
    let mut dp2 = vec![0; n];
    let mut max_price = prices[n - 1];
    for i in (0..n - 1).rev() {
        max_price = max_price.max(prices[i]);
        dp2[i] = dp2[i + 1].max(max_price - prices[i]);
    }

    // 前后交易求和
    let mut max_porfit = 0;
    for i in 0..n {
        max_porfit = max_porfit.max(dp1[i] + dp2[i]);
    }

    max_porfit
}
