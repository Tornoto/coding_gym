/// https://leetcode.com/problems/target-sum/
/// 问题可以转换为背包问题
///
/// 假设添加+的数字的集合为 P(ositive), 添加-的数字集合为 N(egative)
/// 如果能找到这样的添加方式, 可以得到公式：
/// ```
/// sum(P) - sum(N) =  target
/// ```
/// 另外，我们知道这两个集合的总和为数组的总和，即：
/// ```
/// sum(P) + sum(N) = sum(nums)
/// ```
/// 综合这两个公式，我们可以得到
/// ```
/// sum(P)= (sum(nums) + target) / 2
/// ```
/// 如果 `sum(nums) + target` 不是偶数，则说明无法找到对应的方法。
///
/// 问题到这里可以转化为: 将物品恰好放入在容量为 `(sum(nums) + target) / 2` 的背包中的方式。
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum = nums.iter().sum::<i32>();
    // 处理特殊输入
    if sum < target || sum + target < 0 {
        return 0;
    }
    // 处理没有解法的情形
    if (target + sum) % 2 != 0 {
        return 0;
    }
    // 背包容量为 (sum(nums) + target) / 2
    let capacity = ((target + sum) / 2) as usize;

    let n = nums.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];
    // 初始化为1
    dp[0][0] = 1;

    for i in 1..n + 1 {
        let num = nums[i - 1];
        for j in 0..capacity + 1 {
            // 如果不选择 nums[i-1] 的选择方法
            dp[i][j] = dp[i - 1][j];
            if j >= num as usize {
                // 如果容量足够，再补充上选择 nums[i-1] 的方法
                dp[i][j] += dp[i - 1][j - nums[i - 1] as usize];
            }
        }
    }

    dp[n][capacity]
}
