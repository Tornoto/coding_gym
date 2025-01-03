/// https://leetcode.com/problems/coin-change/description/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut dp = vec![i32::MAX; amount as usize + 1];
    dp[0] = 0;

    for index in 1..=amount as usize {
        for &coin in &coins {
            if index as i32 >= coin {
                if dp[index - coin as usize] != i32::MAX {
                    dp[index] = dp[index].min(dp[index - coin as usize] + 1);
                }
            }
        }
    }

    if dp[amount as usize] == i32::MAX {
        return -1;
    }

    dp[amount as usize]
}
