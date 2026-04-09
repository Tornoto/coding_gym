use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/non-decreasing-subsequences/description/
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 二叉树选择
        fn backtrack(
            nums: &[i32],
            start: usize,
            last_picked: i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if start == nums.len() {
                if path.len() > 1 {
                    result.push(path.clone());
                }
                return;
            }

            // 选择当前节点，且符合非降序
            if nums[start] >= last_picked {
                path.push(nums[start]);
                backtrack(&nums, start + 1, nums[start], path, result);
                path.pop();
            }

            // 不选当前节点，且当前节点不是之前处理过的节点
            if nums[start] != last_picked {
                backtrack(nums, start + 1, last_picked, path, result);
            }
        }

        let mut result = vec![];
        let mut path = vec![];

        backtrack(&nums, 0, i32::MIN, &mut path, &mut result);

        result
    }

    pub fn find_subsequences_used_set(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], start: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if path.len() > 1 {
                result.push(path.clone());
            }

            let mut used = vec![false; 201];
            for i in start..nums.len() {
                let num = nums[i];
                if let Some(&last) = path.last() {
                    if last > num {
                        continue;
                    }
                }
                if used[(num + 100) as usize] {
                    continue;
                }

                used[(num + 100) as usize] = true;
                path.push(num);
                backtrack(nums, i + 1, path, result);
                path.pop();
            }
        }

        let mut path = vec![];
        let mut result = vec![];

        backtrack(&nums, 0, &mut path, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subsequences() {
        let nums = vec![4, 6, 7, 7];
        let result = Solution::find_subsequences_used_set(nums);
        println!("{:?}", result);
    }
}
