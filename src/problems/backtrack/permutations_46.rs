use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/permutations/description/
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
                if used[i] {
                    continue;
                }

                path.push(nums[i]);
                used[i] = true;
                backtrack(nums, used, path, result);
                used[i] = false;
                path.pop();
            }
        }
        let mut result = vec![];
        let mut path = Vec::with_capacity(nums.len());
        let mut used = vec![false; nums.len()];
        backtrack(&nums, &mut used, &mut path, &mut result);
        result
    }

    /// 交换法（Swap-based）- 空间优化
    /// 不需要额外的 used 数组
    pub fn permute_swap(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn swap_dfs(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                result.push(nums.clone());
                return;
            }

            for i in start..nums.len() {
                // 选择第 i 个元素放在 start 位置
                nums.swap(start, i);
                // 递归处理后续
                swap_dfs(nums, start + 1, result);
                // 回溯：撤销交换
                nums.swap(start, i);
            }
        }

        let mut nums = nums;
        let mut result = Vec::new();
        swap_dfs(&mut nums, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let res = Solution::permute(vec![1, 2, 3]);
        println!("result: {:?}", res);
    }
}
