/// https://leetcode.com/problems/move-zeroes/description/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut left = 0;

    while left < n && nums[left] != 0 {
        left += 1;
    }

    let mut right = left;

    while left < n && right < n {
        while right < n && nums[right] == 0 {
            right += 1;
        }

        while right < n && nums[right] != 0 {
            nums.swap(left, right);
            left += 1;
            right += 1;
        }
    }
}
