use crate::Solution;

impl Solution {
    /// https://leetcode.cn/problems/squares-of-a-sorted-array/
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];

        if n == 0 {
            return result;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        for idx in (0..nums.len()).rev() {
            let left_square = nums[left] * nums[left];
            let right_square = nums[right] * nums[right];

            if left_square >= right_square {
                result[idx] = left_square;
                left += 1;
            } else {
                result[idx] = right_square;
                right -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-4, -1, 0, 2, 3];
        let result = Solution::sorted_squares(nums);
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }
}
