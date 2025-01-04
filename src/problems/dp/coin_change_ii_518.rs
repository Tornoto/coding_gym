/// https://leetcode.com/problems/coin-change-ii/description/
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    // 很奇怪的设定，不需要换零，次数还得返回 1。
    if amount == 0 {
        return 1;
    }

    let mut dp = vec![0; amount as usize + 1];

    // 这里将 coins 中的 coin 称为“零钱”
    // 将 amount 称为面额
    // 外层遍历每个零钱 内层遍历从 1 到 amount 每种面额
    // 这样，对于每个零钱(coin)，都可以更新一次 amount 的换零方法次数
    // 转移公式: dp[cur_amount] += dp[cur_amount - coin]
    // 即当前面额(cur_amount)，还可以通过 coin 和 cur_amount - coin 组合，
    // 那么当前面额的换零方法数 += (cur_amount - coin) 对应的换零方法数。
    // 假设有输入 coins: [1, 3]， amount 4
    // dp: 0 1 2 3 4
    // coin 1 遍历后
    // dp: 0 1 1 1 1
    for &coin in &coins {
        for cur_amount in 1..=amount as usize {
            // 由于这里dp[0] 初始化为 0 ，如果初始化为 1，就可以统一使用
            // dp[cur_amount] += dp[cur_amount - coin as usize];
            if cur_amount == coin as usize {
                dp[cur_amount] += 1;
            } else if cur_amount > coin as usize {
                dp[cur_amount] += dp[cur_amount - coin as usize];
            }
        }
    }

    dp[amount as usize]
}

/// 另一种写法，代码更简洁，主要是将 dp[0] 初始化为 1
#[allow(unused)]
fn change2(amount: i32, coins: Vec<i32>) -> i32 {
    let mut dp = vec![0; amount as usize + 1];
    dp[0] = 1;

    for &coin in &coins {
        for cur_amount in 1..=amount as usize {
            if cur_amount >= coin as usize {
                dp[cur_amount] += dp[cur_amount - coin as usize];
            }
        }
    }

    dp[amount as usize]
}
