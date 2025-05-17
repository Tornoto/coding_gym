/// https://leetcode.com/problems/minimum-size-subarray-sum/description/
///
/// Given an array of positive integers nums and a positive integer target, return the minimal length of a subarray whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.
///
/// Example 1:
/// - Input: target = 7, nums = [2,3,1,2,4,3]
/// - Output: 2
/// - Explanation: The subarray [4,3] has the minimal length under the problem constraint.
///
/// Example 2:
/// - Input: target = 4, nums = [1,4,4]
/// - Output: 1
///
/// Example 3:
/// - Input: target = 11, nums = [1,1,1,1,1,1,1,1]
/// - Output: 0
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min_len = usize::MAX;
    // start of subarray
    let mut start = 0;
    // end of subarray, exclusive
    let mut end = 1;
    // sum of current subarray
    let mut cur_sum = 0;
    while end <= nums.len() {
        cur_sum += nums[end - 1];
        // if current sum is larger than target,
        // try update min_len and start index of the subarray
        while cur_sum >= target {
            let cur_len = end - start;
            if cur_len < min_len {
                min_len = cur_len;
            }
            cur_sum -= nums[start];
            start += 1;
        }

        end += 1;
    }

    // if min_len is the init value, return 0
    if min_len == usize::MAX {
        return 0;
    }

    min_len as i32
}

#[test]
fn test() {
    let nums = vec![2, 3, 1, 2, 4, 3];
    let target = 7;
    let len = min_sub_array_len(target, nums);
    println!("len: {len}");
}
