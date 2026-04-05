use crate::Solution;

/// https://leetcode.com/problems/combinations/description/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(n: i32, k: i32, start: i32, result: &mut Vec<Vec<i32>>, comb: &mut Vec<i32>) {
            if k == 0 {
                result.push(comb.clone());
                return;
            }

            let end = n - k + 1;
            for start in start..=end {
                comb.push(start);
                backtrack(n, k - 1, start + 1, result, comb);
                comb.pop();
            }
        }
        let mut result = vec![];
        let mut comb = vec![];
        backtrack(n, k, 1, &mut result, &mut comb);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let res = Solution::combine(4, 2);
        assert_eq!(res, vec![[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]);
    }
}
