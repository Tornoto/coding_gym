/// https://leetcode.com/problems/move-zeroes/description/
///
/// use fast-slow pointer to solve this problem
/// - the remove_element can treat as move_target compared to this problem move_zeroes
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut k = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            // use swap
            nums.swap(i, k);
            k += 1;
        }
    }
}
