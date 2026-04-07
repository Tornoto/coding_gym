use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/combination-sum-ii/description/
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidate: &[i32],
            target: i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            let sum: i32 = path.iter().sum();
            if sum > target {
                return;
            }
            if sum == target {
                result.push(path.clone());
                return;
            }

            for (idx, val) in candidate.iter().enumerate() {
                // 跳过相同元素
                if idx > 0 && candidate[idx] == candidate[idx - 1] {
                    continue;
                }
                path.push(*val);
                backtrack(&candidate[idx + 1..], target, path, result);
                path.pop();
            }
        }

        let mut candidates = candidates;
        // 排序
        candidates.sort_unstable();

        let mut path = vec![];
        let mut result = vec![];

        backtrack(&candidates, target, &mut path, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum2() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = Solution::combination_sum2(candidates, target);
        println!(">> {:?}", result);
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let result = Solution::combination_sum2(candidates, target);
        println!(">> {:?}", result);
    }
}
