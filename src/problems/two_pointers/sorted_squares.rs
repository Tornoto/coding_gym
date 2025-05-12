/// https://leetcode.cn/problems/squares-of-a-sorted-array/
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0; n];

    let mut left = 0;
    let mut right = n;
    let mut cur = n - 1;

    while left < right {
        let vl = nums[left] * nums[left];
        let vr = nums[right - 1] * nums[right - 1];

        if vl > vr {
            result[cur] = vl;
            left += 1;
        } else {
            result[cur] = vr;
            right -= 1;
        }
        if cur > 0 {
            cur -= 1;
        }
    }

    result
}

#[test]
fn test() {
    let nums = vec![-4, -1, 0, 2, 3];
    let result = sorted_squares(nums);
    assert_eq!(result, vec![0, 1, 4, 9, 16]);
}
