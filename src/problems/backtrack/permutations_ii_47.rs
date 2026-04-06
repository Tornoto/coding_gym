use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/permutations-ii/description/
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            nums: &[i32],
            used: &mut [bool],
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                result.push(path.clone());
                return;
            }

            for i in 0..nums.len() {
                // 剪枝1：跳过已经使用的元素
                if used[i] {
                    continue;
                }

                // 剪枝2：如果当前元素与前一个相同，且前一个未被使用，说明是同一层的重复，跳过
                // i > 0 && nums[i] == nums[i-1] && !used[i-1]  → 同一树层去重
                // i > 0 && nums[i] == nums[i-1] && used[i-1]   → 同一树枝可用（允许重复元素在不同位置）
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                    continue;
                }
                // 选择
                used[i] = true;
                path.push(nums[i]);

                // 递归
                backtrack(nums, used, path, result);

                // 撤销选择（回溯）
                path.pop();
                used[i] = false;
            }
        }

        let mut result = Vec::new();
        let mut nums = nums;
        // 关键：排序让相同元素相邻，便于去重
        nums.sort();
        let mut path = Vec::new();
        let mut used = vec![false; nums.len()];

        backtrack(&nums, &mut used, &mut path, &mut result);
        result
    }

    /// swap-based
    pub fn permute_unique_swap(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
            // 终止条件
            if start == nums.len() {
                result.push(nums.clone())
            }

            for i in start..nums.len() {
                // 去重：如果当前元素在 [start, i) 范围内已出现过，跳过
                // 避免同一位置重复使用相同值的元素
                if (start..i).any(|j| nums[j] == nums[i]) {
                    continue;
                }
                // 交换：将 nums[i] 放到 start 位置
                nums.swap(start, i);
                // 递归处理下一个位置
                backtrack(nums, start + 1, result);
                // 回溯：恢复交换
                nums.swap(start, i);
            }
        }

        let mut result = Vec::new();
        let mut nums = nums;
        // 排序用于去重
        nums.sort();
        backtrack(&mut nums, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 2];
        let result = Solution::permute_unique(nums);
        println!("{:?}", result);
        let nums = vec![1, 1, 1, 2];
        let result = Solution::permute_unique_swap(nums);
        println!("{:?}", result);
    }
}
