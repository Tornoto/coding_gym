use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/combination-sum/description/
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidates: &[i32],
            target: i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            let sum: i32 = path.iter().sum();
            // 如果凑够target则加入结果集，并返回
            if sum > target {
                return;
            }

            // 如果和超出target则返回
            if sum == target {
                result.push(path.clone());
                return;
            }

            // 注意candidates中的元素可以重复选取
            // 因此递归调用时还是从i开始
            for i in 0..candidates.len() {
                path.push(candidates[i]);
                backtrack(&candidates[i..], target, path, result);
                path.pop();
            }
        }

        let mut result = vec![];
        let mut path = vec![];

        backtrack(&candidates, target, &mut path, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = Solution::combination_sum(candidates, target);
        println!("{:?}", result);

        let candidates = vec![2, 3, 5];
        let target = 8;
        let res = Solution::combination_sum(candidates, target);
        println!("{:?}", res);
    }
}
