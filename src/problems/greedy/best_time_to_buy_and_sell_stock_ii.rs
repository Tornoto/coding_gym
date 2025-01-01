pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;

    for idx in 0..prices.len() - 1 {
        let diff = prices[idx + 1] - prices[idx];
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
