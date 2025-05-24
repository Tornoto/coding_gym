use std::collections::HashMap;

/// https://leetcode.com/problems/valid-anagram/description/
/// 假设只包含小写英文字符
pub fn is_anagram(s: String, t: String) -> bool {
    let mut map1 = vec![0; 26];
    let mut map2 = vec![0; 26];

    for ch in s.chars() {
        map1[ch as usize - 'a' as usize] += 1;
    }
    for ch in t.chars() {
        map2[ch as usize - 'a' as usize] += 1;
    }

    map1 == map2
}

/// 假设包含 Unicode 字符
pub fn is_anagram_unicode(s: String, t: String) -> bool {
    let mut ms = HashMap::with_capacity(s.len());

    for ch in s.chars() {
        ms.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }

    for ch in t.chars() {
        ms.entry(ch).and_modify(|count| *count -= 1).or_insert(-1);
    }

    ms.into_values().all(|v| v == 0)
}
