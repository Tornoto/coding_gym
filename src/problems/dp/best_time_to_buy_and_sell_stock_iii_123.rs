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

/// 根据AI提示，提供高效状态机动态规划解法
///
/// 我们用几个变量来表示当前的状态：
/// - buy1: 第一次买入后的最大利润（可以是负数）。
/// - sell1: 第一次卖出后的最大利润。
/// - buy2: 第二次买入后的最大利润。
/// - sell2: 第二次卖出后的最大利润。
///
/// 这些状态变量会随着遍历价格数组而更新，最终 sell2 就是我们要求的最大利润。
///
/// 状态转移方程
///
/// - 第一次买入:
///     - `buy1 = buy1.max(-price)`
///     - 要么保持之前的买入状态，要么以当前价格买入（初始资金为 0，因此买入后利润为 -price）。
/// - 第一次卖出:
///     - `sell1 = sell1.max(buy1 + price)`
///     - 要么保持之前的卖出状态，要么以当前价格卖出（利润为 buy1 + price）。
/// - 第二次买入:
///     - `buy2 = buy2.max(sell1 - price)`
///     - 要么保持之前的买入状态，要么以当前价格买入（利润为 sell1 - price）。
/// - 第二次卖出:
///     - `sell2 = sell2.max(buy2 + price)`
///     - 要么保持之前的卖出状态，要么以当前价格卖出（利润为 buy2 + price）。
///
/// 初始化
/// - buy1 和 buy2 初始化为 i32::MIN，表示尚未买入。
/// - sell1 和 sell2 初始化为 0，表示尚未卖出。
///
/// 最大利润存储在 sell2 中。
///
/// TODO
/// 1. 为什么代码看起来可以在一天内多次交易？
/// 2. 如何掌握和理解这些变量？
///
pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0; // 边界条件：空数组
    }

    // 第一次买入后的最大利润
    let mut buy1 = i32::MIN;
    // 第一次卖出后的最大利润
    let mut sell1 = 0;
    // 第二次买入后的最大利润
    let mut buy2 = i32::MIN;
    // 第二次卖出后的最大利润
    let mut sell2 = 0;

    for price in prices {
        // 更新第一次买入和卖出
        // 第一次买入
        buy1 = buy1.max(-price);
        // 第一次卖出
        sell1 = sell1.max(buy1 + price);

        // 更新第二次买入和卖出
        // 第二次买入
        buy2 = buy2.max(sell1 - price);
        // 第二次卖出
        sell2 = sell2.max(buy2 + price);
    }

    // 最大利润
    sell2
}
