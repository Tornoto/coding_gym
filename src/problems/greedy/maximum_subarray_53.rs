/// https://leetcode.com/problems/maximum-subarray/description/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    use std::i32;
    // max_sum 初始化为 i32:MIN 可以处理全负数输入
    let mut max_sum = i32::MIN;
    let mut sum = 0;

    for val in nums {
        // 更新局部和
        sum += val;
        // 如果局部和大于当前最大和，则更新最大和
        if sum > max_sum {
            max_sum = sum;
        }
        // 如果局部和小于0则立刻丢弃
        if sum < 0 {
            sum = 0;
        }
    }

    max_sum
}
