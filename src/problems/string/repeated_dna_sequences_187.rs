use std::collections::HashMap;

/// https://leetcode.com/problems/repeated-dna-sequences/
///
/// 看到字符串比较，就容易联想到一些字符串比较的算法。
///
/// 需要注意的是这里有两个关键词：次数、固定长度。
///
/// 因此可以遍历一遍字符串，统计窗口划过的子字符串出现的次数
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() <= 10 {
        return vec![];
    }
    let mut map: HashMap<&str, i32> = HashMap::new();

    let mut res = vec![];
    for i in 0..=s.len() - 10 {
        map.entry(&s[i..i + 10])
            .and_modify(|count| {
                if *count == 1 {
                    res.push(s[i..i + 10].to_string());
                }
                *count += 1;
            })
            .or_insert(1);
    }

    res
}
