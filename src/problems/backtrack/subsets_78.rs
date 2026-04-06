use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/subsets/description/
    /// 递归法求解
    pub fn subsets_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            nums: &[i32],
            start: usize,
            subset: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(subset.clone());

            for idx in start..nums.len() {
                subset.push(nums[idx]);
                // 递归时从下一个位置开始，避免重复选择
                backtrack(nums, idx + 1, subset, result);
                subset.pop();
            }
        }

        let mut result = vec![];
        let mut subset = vec![];
        backtrack(&nums, 0, &mut subset, &mut result);

        result
    }

    /// 迭代法求解
    pub fn subsets_iter(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        result.push(vec![]);

        for num in nums {
            let mut result_clone = result.clone();
            for sub_res in result_clone.iter_mut() {
                sub_res.push(num);
            }
            result.extend(result_clone);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subsets() {
        let nums = vec![1, 2, 3];
        // let result = subsets(nums);
        let result = Solution::subsets_recursive(nums);
        println!("{:?}", result);
    }
}
