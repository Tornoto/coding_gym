/// https://leetcode.com/problems/wiggle-subsequence/description/
/// 贪心解法
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut cur_diff;
    let mut pre_diff = 0;
    let mut count = 1;

    for idx in 0..nums.len() - 1 {
        cur_diff = nums[idx + 1] - nums[idx];

        if (cur_diff > 0 && pre_diff <= 0) || (cur_diff < 0 && pre_diff >= 0) {
            count += 1;
            pre_diff = cur_diff;
        }
    }

    count
}

/// 动态规划解法
pub fn wiggle_max_length_dp(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let n = nums.len();

    // up[i]: 以nums[i] 结尾的 wiggle_seq 最大长度，且 nums[i] 是峰
    let mut up: Vec<i32> = vec![1; n];
    // down[i]: 以nums[i] 结尾的 wiggle_seq 最大长度，且 nums[i] 是谷
    let mut down: Vec<i32> = vec![1; n];

    for idx in 1..n {
        if nums[idx] > nums[idx - 1] {
            up[idx] = down[idx - 1] + 1;
            down[idx] = down[idx - 1];
        } else if nums[idx] < nums[idx - 1] {
            down[idx] = up[idx - 1] + 1;
            up[idx] = up[idx - 1];
        } else {
            up[idx] = up[idx - 1];
            down[idx] = down[idx - 1];
        }
    }

    std::cmp::max(up[n - 1], down[n - 1])
}
