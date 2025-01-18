/// https://leetcode.cn/problems/perfect-squares/description/
pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;

    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0;

    let nums = (n as f64).sqrt() as usize;

    for i in 1..nums + 1 {
        for j in i * i..n + 1 {
            dp[j] = dp[j].min(dp[j - i * i] + 1);
        }
    }

    dp[n]
}
