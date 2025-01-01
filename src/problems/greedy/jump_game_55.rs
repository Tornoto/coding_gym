#![allow(unused)]

/// https://leetcode.cn/problems/jump-game/description/
/// 不应该拘泥于能否从一个点跳到另一个点，而是这个点的辐射范围
pub fn can_jump(nums: Vec<i32>) -> bool {
    // can_jum_helper(&nums)

    if nums.is_empty() {
        return false;
    }

    // 默认辐射范围是 1。输入为 [0] 时，即已经到末尾
    // 对于 nums: [2, 1]
    // nums[0] 的辐射范围是当前下标及向后 2 个元素，可以辐射以 2 开头，长度为 3 的数组，即 [2, 1, x]。
    let mut coverage = 1;

    // 遍历数组，更新辐射范围
    for (index, value) in nums.iter().enumerate() {
        // 如果当前标不在辐射范围内，说明不可达，直接返回
        if index > coverage - 1 {
            return false;
        }

        // 如果当前位置可达，且可以扩大辐射范围，则更新辐射范围
        if index + 1 + *value as usize > coverage {
            coverage = index + 1 + *value as usize;
        }

        // 符合条件，则提前退出
        if coverage >= nums.len() {
            return true;
        }
    }

    // 如果辐射范围大于数组长度，则说明可以从起始位置跳到结尾
    coverage >= nums.len()
}

/// 递归算法（会超时）
fn can_jum_helper(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let first = nums[0];

    if first == 0 {
        return false;
    }

    if first as usize >= nums.len() - 1 {
        return true;
    }

    let mut reach_end = false;

    for next_idx in (1..=first as usize).rev() {
        reach_end = reach_end || can_jum_helper(&nums[next_idx..]);
        if reach_end {
            return true;
        }
    }

    reach_end
}
