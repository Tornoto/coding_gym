use std::collections::HashMap;

use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn backtrack(
            n: usize,
            digits: &str,
            map: &HashMap<char, Vec<char>>,
            comb: &mut String,
            result: &mut Vec<String>,
        ) {
            if comb.len() == n {
                result.push(comb.clone());
                return;
            }

            if let Some(ch) = digits.chars().next() {
                if let Some(letters) = map.get(&ch) {
                    for letter in letters {
                        comb.push(*letter);
                        backtrack(n, &digits[1..], &map, comb, result);
                        comb.pop();
                    }
                }
            }
        }

        if digits.is_empty() {
            return vec![];
        }

        let map = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        let mut result = vec![];
        let mut comb = String::with_capacity(digits.len());

        backtrack(digits.len(), &digits, &map, &mut comb, &mut result);
        result
    }

    pub fn letter_combinations_iter(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digits: Vec<usize> = digits
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .collect();

        // 数字到字母的映射
        let phone_map = vec![
            "",     // 0
            "",     // 1
            "abc",  // 2
            "def",  // 3
            "ghi",  // 4
            "jkl",  // 5
            "mno",  // 6
            "pqrs", // 7
            "tuv",  // 8
            "wxyz", // 9
        ];

        let mut result = vec![String::new()];

        for digit in digits {
            let letters = phone_map[digit];
            let mut next_result = Vec::new();

            // 对当前每个部分结果，追加当前数字的所有可能字母
            for prefix in &result {
                for &ch in letters.as_bytes() {
                    let mut new_comb = prefix.clone();
                    new_comb.push(ch as char);
                    next_result.push(new_comb);
                }
            }
            // 滚动更新
            result = next_result;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        // Input: digits = "23"
        // Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
        let digits = "23";
        let result = Solution::letter_combinations_iter(digits.to_string());
        println!("{:?}", result);
    }
}
