/// https://leetcode.cn/problems/sort-array-by-parity-ii/description/
pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut even_index = 0;
    let mut odd_index = 1;
    let n = nums.len();

    while even_index < n && odd_index < n {
        if nums[even_index] % 2 == 0 {
            even_index += 2;
        } else if nums[odd_index] % 2 == 1 {
            odd_index += 2;
        } else {
            nums.swap(even_index, odd_index);
            even_index += 2;
            odd_index += 2;
        }
    }
    nums
}
