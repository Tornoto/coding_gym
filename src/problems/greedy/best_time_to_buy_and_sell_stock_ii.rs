/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;

    for idx in 0..prices.len() - 1 {
        let diff = prices[idx + 1] - prices[idx];
        // 总是考虑上升的部分即可
        // 假设有两天价格为：[1,3], 3 后有两种可能：
        // 小于3：则等价于在3的时候卖掉，后续在 diff>0 的 idx 日买入
        // 大于等于3：继续持有，累计利润
        if diff >= 0 {
            profit += diff;
        }
    }

    profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit = max_profit(prices);
        assert_eq!(profit, 7);

        let prices = vec![1, 2, 3, 4, 5];
        let profit = max_profit(prices);
        assert_eq!(profit, 4);

        let prices = vec![7, 6, 4, 3, 1];
        let profit = max_profit(prices);
        assert_eq!(profit, 0);
    }
}
