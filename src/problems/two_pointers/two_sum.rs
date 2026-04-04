use std::collections::HashMap;

use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/two-sum/description/
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (idx, ele) in nums.iter().enumerate() {
            if let Some(idx_prev) = map.get(ele) {
                return vec![*idx_prev as i32, idx as i32];
            }
            map.insert(target - *ele, idx);
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
