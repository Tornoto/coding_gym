/// https://leetcode.com/problems/house-robber-ii/description/
/// 成环的情形，考虑：不处理第一个元素和不处理最后一个元素的情形
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }

    let n = nums.len();
    // 分别计算不抢第一户和不抢最后一户的情况
    let first = rob_helper(&nums[0..n - 1]);
    let last = rob_helper(&nums[1..]);

    // 取大值
    first.max(last)
}

fn rob_helper(nums: &[i32]) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut prev1 = 0;
    let mut prev2 = 0;

    for num in nums {
        let cur = prev1.max(prev2 + num);
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}
