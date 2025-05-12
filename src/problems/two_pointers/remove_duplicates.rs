/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
///
/// use fast-slow pointer to solve this problem
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[k] {
            k += 1;
            nums[k] = nums[i];
        }
    }

    // return the length not index
    (k + 1) as i32
}
