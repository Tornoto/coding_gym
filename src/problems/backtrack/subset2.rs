use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/subsets-ii/description/
    pub fn subsets_with_dup_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            nums: &[i32],
            start: usize,
            subset: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(subset.clone());

            for idx in start..nums.len() {
                if idx > start && nums[idx] == nums[idx - 1] {
                    continue;
                }
                subset.push(nums[idx]);
                backtrack(nums, idx + 1, subset, result);
                subset.pop();
            }
        }

        let mut nums = nums;
        // 排序
        nums.sort();

        let mut result = vec![];
        let mut subset = vec![];

        backtrack(&nums, 0, &mut subset, &mut result);

        result
    }

    pub fn subsets_with_dup_iter(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 排序
        nums.sort();
        let mut result = vec![vec![]];
        // 记录上一轮新增子集的起始索引
        let mut start_idx = 0;

        for i in 0..nums.len() {
            // 如果是重复元素，只能从上一轮新增的子集开始扩展
            let begin = if i > 0 && nums[i] == nums[i - 1] {
                start_idx
            } else {
                0
            };

            let end = result.len();
            // 更新边界：本轮新增的子集将从 end 开始
            start_idx = end;

            // 遍历指定范围，生成新子集
            for j in begin..end {
                let mut new_subset = result[j].clone();
                new_subset.push(nums[i]);
                result.push(new_subset);
            }
        }

        result
    }
}
