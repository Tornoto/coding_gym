pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;

    // 记录局部利润
    let mut sum = 0;
    for idx in 0..prices.len() - 1 {
        let diff = prices[idx + 1] - prices[idx];
        sum += diff;
        // 如果局部利润大于当前最大利润，则更新最大利润
        if sum > profit {
            profit = sum;
        }
        // 如果局部利润小于0，则立刻丢弃该局部利润
        // 在局部利润小于0前，允许某几日亏损，这样做可以等到某个暴赚的日子
        // 另外，在这个过程中，一旦出现高于当前最大利润的情形，会更新最大利润；
        // 即使后续有几日亏损，也不影响此前对最大利润的更新
        if sum < 0 {
            sum = 0;
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
        assert_eq!(profit, 5);

        let prices = vec![7, 6, 4, 3, 1];
        let profit = max_profit(prices);
        assert_eq!(profit, 0);
    }
}
