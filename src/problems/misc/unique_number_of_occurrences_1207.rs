use std::collections::{HashMap, HashSet};

/// https://leetcode.com/problems/unique-number-of-occurrences/description/
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let n = arr.len();
    let mut map = HashMap::with_capacity(n);
    let mut set = HashSet::with_capacity(n);

    for &num in &arr {
        map.entry(num).and_modify(|e| *e += 1).or_insert(1);
    }

    for (_, val) in map {
        if set.contains(&val) {
            return false;
        } else {
            set.insert(val);
        }
    }

    true
}

/// 由于限定数字在`[-1000,1000]` 可以用一个长度为2001的数组作为哈希表
pub fn unique_occurrences_v2(arr: Vec<i32>) -> bool {
    let mut frequence = vec![0; 2001];

    for &num in &arr {
        frequence[(num + 1000) as usize] += 1;
    }
    // 由于数组长度小于1000，因此最大出现次数不会超过1000
    // repeat 数组记录每个出现次数 注意长度
    let mut repeat = vec![false; 1001];

    for num in frequence {
        // 如果频率为0，不考虑
        if num == 0 {
            continue;
        }

        if repeat[num] {
            return false;
        }
        repeat[num] = true;
    }

    true
}
