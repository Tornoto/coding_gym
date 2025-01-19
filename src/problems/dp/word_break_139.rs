/// https://leetcode.cn/problems/word-break/description/
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();

    let mut dp = vec![false; n + 1];
    // 空字符串可以被成功拆分。
    dp[0] = true;

    // 外层遍历背包 对于 applepenapple 这种案例 需要重复使用 apple
    // 因此外层需要遍历背包
    for j in 1..n + 1 {
        for word in &word_dict {
            let wlen = word.len();
            if j >= wlen {
                let start = j - wlen;
                let end = j;
                // 如果当前位置和词典中的对上了
                let matched = word == &s[start..end];
                // 除了要求这个词自身对上了，还得要求他前面的也对上了即 dp[j - wlen]
                // 如果匹配上就置为 true 因为word_dict没有重复字符串，因此后续无需再比较
                // 从而优化循环
                if matched && dp[start] {
                    dp[j] = true;
                    break;
                }
                // 另一种欠优化的更新方式
                // let matched = word == &s[start..end];
                // 除了要求这个词自身对上了，还得要求他前面的也对上了即 dp[j - wlen]
                // 如果没对上 就使用 dp[i-1][j] 即 (dp[j] || ...)
                // dp[j] = dp[j] || (dp[start] && matched);
            }
        }
    }

    dp[n]
}

#[test]
fn test() {
    let word_dict = vec!["cat", "dot"];
    let word_dict = word_dict.iter().map(|&word| word.to_string()).collect();
    let s = "catdog".to_string();
    let result = word_break(s, word_dict);
    println!("{:?}", result);

    let word_dict = vec!["apple", "pen"];
    let word_dict = word_dict.iter().map(|&word| word.to_string()).collect();
    let s = "applepenapple".to_string();
    let result = word_break(s, word_dict);
    println!("{:?}", result);
}
