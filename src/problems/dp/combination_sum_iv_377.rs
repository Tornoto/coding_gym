/// https://leetcode.cn/problems/combination-sum-iv/description/
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();

    if n == 0 || target <= 0 {
        return 0;
    }

    let capacity = target as usize;
    let mut dp = vec![0; capacity + 1];
    dp[0] = 1;

    // 需要区分排列和组合。
    // 对于组合场景，需要先遍历物品，再遍历容量；
    // 对于排列场景，需要先遍历容量，再遍历物品。
    for i in (1..capacity + 1).rev() {
        for &num in &nums {
            if i >= num as usize {
                dp[i] += dp[i - num as usize];
            }
        }
    }

    dp[capacity]
}
