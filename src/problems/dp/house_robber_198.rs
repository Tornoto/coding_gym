/// https://leetcode.com/problems/house-robber/description/
/// 提供多个版本，以体现优化的思路
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }

    let n = nums.len();
    // 使用数组 dp[0] 取 nums[0]
    // dp[1] 取 nums[0] 或 nums[1] 中的大值
    let mut dp = vec![0; n];
    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);

    for i in 2..n {
        // 如果打劫 i 户, 则不打劫 i-2 户, 收获: dp[i - 2] + nums[i]
        // 如果不打劫 i 户, 则此前最大收获 dp[i-1]
        // 可以看到 dp[i] 依赖 dp[i-1] 和 dp[i-2]
        // 可以使用两个变量迭代更新 详见下个版本
        dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
    }

    dp[n - 1]
}

pub fn rob_opt(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    // 当前的前一个
    let mut prev1 = 0;
    // 当前的前两个
    let mut prev2 = 0;

    for num in nums {
        // 迭代更新 prev1 和 prev2
        // cur 取 prev1 (不抢劫当前户的收获) 和 [prev2 + num](抢当前户)
        // 中的大值
        let cur = prev1.max(prev2 + num);
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}
