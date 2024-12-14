use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut max_len = 0;
    let mut cur_len = 0;
    let mut start = 0;
    for (idx, ch) in s.chars().enumerate() {
        // if find element
        if let Some(pre_idx) = map.get(&ch) {
            if cur_len > max_len {
                max_len = cur_len;
            }
            if *pre_idx as i32 > start {
                start = *pre_idx;
            }
            cur_len = idx as i32 - start;
            map.insert(ch, idx as i32);
        } else {
            map.insert(ch, idx as i32);
            cur_len += 1;
        }
    }
    if cur_len > max_len {
        return cur_len;
    } else {
        return max_len;
    }
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(length_of_longest_substring("abba".to_string()), 2);
}