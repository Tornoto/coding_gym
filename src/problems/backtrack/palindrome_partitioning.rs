use crate::Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn backtrack(s: &str, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
            if s.is_empty() {
                result.push(path.clone());
                return;
            }

            for i in 0..s.len() {
                let slice = &s[0..=i];
                if is_palindrome(slice) {
                    path.push(slice.to_string());
                    backtrack(&s[i + 1..], path, result);
                    path.pop();
                }
            }
        }

        fn is_palindrome(s: &str) -> bool {
            let bytes = s.as_bytes();
            let (mut left, mut right) = (0, bytes.len());
            while left < right {
                if bytes[left] != bytes[right - 1] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        }

        let mut path = Vec::new();
        let mut result = Vec::new();

        backtrack(&s, &mut path, &mut result);

        result
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let s = "aab";
        let result = Solution::partition(s.to_string());
        println!(">> {:?}", result);
    }
}
