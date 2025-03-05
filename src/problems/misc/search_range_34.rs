/// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first_position = -1;
    let mut last_position = -1;

    // Binary search for the first position
    let (mut left, mut right) = (0, nums.len() as i32 - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            first_position = mid;
            right = mid - 1;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    // Binary search for the last position
    let (mut left, mut right) = (0, nums.len() as i32 - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            last_position = mid;
            left = mid + 1;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    vec![first_position, last_position]
}
