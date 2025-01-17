/// https://leetcode.com/problems/target-sum/
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum = nums.iter().sum::<i32>();
    if (target + sum) % 2 != 0 {
        return 0;
    }
    let capacity = ((target + sum) / 2) as usize;

    let n = nums.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    dp[0][0] = 1;

    for i in 1..n + 1 {
        let num = nums[i - 1];
        for j in 0..capacity + 1 {
            dp[i][j] = dp[i - 1][j];
            if j >= num as usize {
                dp[i][j] += dp[i - 1][j - nums[i - 1] as usize];
            }
        }
    }

    dp[n][capacity]
}
