/// https://leetcode.com/problems/partition-equal-subset-sum/description/
pub fn can_partition(nums: Vec<i32>) -> bool {
    // 如果可分成等和的子集，则数组和必为偶数
    let sum = nums.iter().sum::<i32>();
    if sum % 2 != 0 {
        return false;
    }

    // 转化为0-1背包问题
    // 即背包容量为 sum/2 时，能放下多少物品
    // 如果容量和物品总量相等 则说明可以拆分成等和的子集
    let capacity = sum as usize / 2;
    let mut dp = vec![0; capacity + 1];

    for num in nums {
        for i in (num as usize..capacity + 1).rev() {
            dp[i] = dp[i].max(dp[i - num as usize] + num)
        }
    }

    dp[capacity] == sum / 2
}

/// 另一种思路
pub fn can_partition_v2(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % 2 != 0 {
        return false;
    }

    let capacity = sum as usize / 2;
    // dp[i] 定义为: 前 num 个物品能否填满容量为 i 的背包
    let mut dp = vec![false; capacity + 1];
    // 空集合置为true
    dp[0] = true;

    for num in nums {
        for i in (num as usize..capacity + 1).rev() {
            dp[i] = dp[i] || dp[i - num as usize]
        }
    }

    return dp[capacity];
}
