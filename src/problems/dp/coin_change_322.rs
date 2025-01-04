/// https://leetcode.com/problems/coin-change/description/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut dp = vec![i32::MAX; amount as usize + 1];
    dp[0] = 0;

    // 这里将 coins 中的 coin 称为零钱，将 amount 称为面额
    // 外层遍历从 1 到 amount 的面额，内层遍历零钱
    // 这样，对于当前面额，遍历一遍所有零钱。
    // 由于遍历当前面额时，之前面额的最少换零次数已经确定下来。
    // 对于当前面额，可以根据当前 零钱 和 cur_amount - coin 面额的最少换零次数
    // 判断是否更新其换零次数。
    // 转移公式:
    // dp[cur_amount] = MIN { dp[cur_amount], dp[cur_amount - coin as usize] + 1 }
    for cur_amount in 1..=amount as usize {
        for &coin in &coins {
            if cur_amount as i32 >= coin {
                if dp[cur_amount - coin as usize] != i32::MAX {
                    dp[cur_amount] = dp[cur_amount].min(dp[cur_amount - coin as usize] + 1);
                }
            }
        }
    }

    if dp[amount as usize] == i32::MAX {
        return -1;
    }

    dp[amount as usize]
}
