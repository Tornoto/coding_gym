/// https://leetcode.com/problems/move-zeroes/description/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut left = 0;

    // 找到第一个0
    while left < n && nums[left] != 0 {
        left += 1;
    }

    let mut right = left;

    while left < n && right < n {
        // 找到0之后第一个非0，考虑有连续0的情形
        while right < n && nums[right] == 0 {
            right += 1;
        }
        // 交换，并同时移动指针
        while right < n && nums[right] != 0 {
            nums.swap(left, right);
            left += 1;
            right += 1;
        }
    }
}

/// 一个更优雅的解法 快慢指针
pub fn move_zeroes_fast_slow(nums: &mut Vec<i32>) {
    let mut slow = 0;
    for fast in 0..nums.len() {
        if nums[fast] != 0 {
            nums.swap(slow, fast);
            slow += 1;
        }
    }
}
