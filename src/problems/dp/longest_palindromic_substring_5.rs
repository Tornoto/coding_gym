/// https://leetcode.com/problems/longest-palindromic-substring/
pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut dp = vec![vec![false; n]; n];
    let mut max_len = 0;
    let mut start = 0;
    let mut end = 0;

    // 动态规划填表：按子串长度从小到大处理
    for len in 1..=n {
        for i in 0..n - len + 1 {
            let j = i + len - 1;
            if len == 1 {
                dp[i][j] = true;
            } else if len == 2 {
                dp[i][j] = chars[i] == chars[j];
            } else {
                dp[i][j] = chars[i] == chars[j] && dp[i + 1][j - 1];
            }

            // 更新最长回文记录
            if dp[i][j] && len > max_len {
                max_len = len;
                start = i;
                end = j;
            }
        }
    }

    chars[start..=end].iter().collect()
}
