use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/combination-sum-iii/description/
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidates: &[i32],
            k: i32,
            n: i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            let sum: i32 = path.iter().sum();
            if sum > n {
                return;
            }
            if path.len() as i32 == k && sum == n {
                result.push(path.clone());
                return;
            }

            for i in 0..candidates.len() {
                path.push(candidates[i]);
                backtrack(&candidates[i + 1..], k, n, path, result);
                path.pop();
            }
        }

        // 只能选取1..=9的
        let candidates: Vec<i32> = (1..=9).collect();
        let mut result = vec![];
        let mut path = vec![];

        backtrack(&candidates, k, n, &mut path, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_combination_sum3() {
        // 输入: k = 3, n = 7
        // 输出: [[1,2,4]]
        let result = Solution::combination_sum3(3, 7);
        println!("res: {:?}", result);

        // 输入: k = 3, n = 9
        // 输出: [[1,2,6], [1,3,5], [2,3,4]]
        let result = Solution::combination_sum3(3, 9);
        println!("res: {:?}", result);
    }
}
