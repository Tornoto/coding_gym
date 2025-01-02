/// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/description/
pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let mut k = k;

    // 排序
    nums.sort();

    for value in nums.iter_mut() {
        // 如果有负数，在k个范围内，逐个将其置为正数
        if *value < 0 {
            *value = -1 * *value;
            k -= 1;
        } else {
            // 如果遇到正数，说明没有负数，跳出
            break;
        }
        // 如果k耗尽，跳出
        if k == 0 {
            break;
        }
    }

    // 此时，如果 k>0，数组中必然无负数
    // 如果 k 是偶数，无需处理，负负得正；
    // 如果 k 是奇数，将最小的整数置为负数
    if k % 2 == 1 {
        if let Some(value) = nums.iter_mut().min() {
            *value = -1 * *value;
        }
    }

    nums.iter().sum()
}
