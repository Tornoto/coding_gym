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

/// 上面“持有现金”的理解方式，对于从前向后的遍历方式来说较好理解，但对于从后向前的遍历方式，不便理解。
///
/// 观察”当天持有股票时拥有的现金量“对应的转移公式
///
/// `dp[i].0 = dp[i - 1].0.max(-prices[i])`
///
/// 事实上这是不断获取当前最小价格的过程。
///
/// 而”当前不持有股票拥有的现金量“对应的转移公式
///
/// `dp[i].1 = dp[i - 1].1.max(prices[i] + dp[i - 1].0)`
///
/// 上面提到`dp[i].0`实际上是当前的最小价格，因此可以改写为：
///
/// `dp[i].1 = dp[i - 1].1.max(prices[i] - min_price)`
///
/// 使用当前价格减去当前最小价格，更新最大利润。这样我们可以再写一版本
pub fn max_profit_forward(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![0; n];
    let mut min_price = prices[0];
    for i in 1..n {
        min_price = min_price.min(prices[i]);
        dp[i] = dp[i - 1].max(prices[i] - min_price)
    }

    dp[n - 1]
}

/// 下面再提供一种从后向前遍历的方法
///
/// 如果从后向前遍历，就需要保留”后面已经出现的最大值“`max_price`
///
/// 当前的利润是`max_price-price[i]`
pub fn max_profit_backward(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![0; n];
    let mut max_price = prices[n - 1];
    for i in (0..n - 1).rev() {
        max_price = max_price.max(prices[i]);
        dp[i] = dp[i + 1].max(max_price - prices[i])
    }

    dp[0]
}
