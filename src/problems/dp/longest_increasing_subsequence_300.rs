/// https://leetcode.com/problems/longest-increasing-subsequence/
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n + 1];

    for i in 1..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    dp.iter().max().unwrap().clone()
}

#[test]
fn test() {
    let nums = vec![6, 6, 6, 6];
    let length = length_of_lis(nums);
    println!("{:?}", length);
}
