use std::collections::HashMap;

/// https://leetcode.com/problems/partition-labels/description/
pub fn partition_labels(s: String) -> Vec<i32> {
    // letter-pos map, key: char, pos: last shown index of the letter
    // 记录字符最后出现的位置（称之为某个字符的覆盖范围）
    let mut lp_map: HashMap<char, usize> = HashMap::new();

    for (idx, ch) in s.chars().enumerate() {
        lp_map
            .entry(ch)
            .and_modify(|pre_idx| *pre_idx = idx)
            .or_insert(idx);
    }

    let mut result = vec![];
    // 某个片段预估的覆盖范围
    let mut coverage = 0;
    // 记录片段长度
    let mut partition_len = 0;
    for (idx, ch) in s.chars().enumerate() {
        if let Some(pos) = lp_map.get(&ch) {
            partition_len += 1;
            // 如果当前预估覆盖范围内的某个字符的覆盖范围更广，则更新覆盖范围
            if *pos > coverage {
                coverage = *pos;
            }
            // 如果到达覆盖范围边界，则保留当前片段长度
            if idx == coverage {
                result.push(partition_len);
                partition_len = 0;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = "ababcbacadefegdehijhklij";
        let result = partition_labels(s.to_string());
        assert_eq!(result, vec![9, 7, 8]);

        let s = "eccbbbbdec";
        let result = partition_labels(s.to_string());
        assert_eq!(result, vec![10]);
    }
}
