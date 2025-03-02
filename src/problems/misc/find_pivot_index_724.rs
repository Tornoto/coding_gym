/// https://leetcode.com/problems/find-pivot-index/
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let sum: i32 = nums.iter().sum();

    let mut left_sum = 0;
    for i in 0..n {
        if left_sum == sum - nums[i] - left_sum {
            return i as i32;
        }
        left_sum += nums[i];
    }

    return -1;
}
