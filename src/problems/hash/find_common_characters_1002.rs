/// https://leetcode.com/problems/find-common-characters/
pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let len = 26;
    // 基线map，用于保留目前遍历的字符串公共字符出现的最小次数
    let mut map_base = vec![0; len];
    let mut map = vec![0; len];

    for i in 0..words.len() {
        let word = &words[i];
        // i==0时 第一个字符串的字符频率统计入基线map
        if i == 0 {
            for ch in word.chars() {
                map_base[ch as usize - 'a' as usize] += 1;
            }
        } else {
            // 统计当前字符串中每个字符出现频率
            for ch in word.chars() {
                map[ch as usize - 'a' as usize] += 1;
            }
            // 比较当前和基线map中，每个字符的出现频率，总是取小的那个
            for i in 0..len {
                map_base[i] = map_base[i].min(map[i]);
                map[i] = 0;
            }
        }
    }

    let mut result = vec![];
    for i in 0..len {
        for _ in 0..map_base[i] {
            result.push(((i as u8 + 'a' as u8) as char).into());
        }
    }

    result
}
