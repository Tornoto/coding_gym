use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for word in strs {
        // 将字符串排序后作为key，eat 和 ate 都转化为同一个key：aet
        let mut sorted = word.clone().into_bytes();
        sorted.sort();
        // 如果key对应的value不存在，则插入空vec
        // or_insert 返回的是可变value，然后push当前字母异位词
        map.entry(sorted).or_insert(vec![]).push(word);
    }
    map.into_values().collect()
}

#[test]
fn test_group_anagrams(){
    let strs = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
    let res = group_anagrams(strs);
    assert_eq!(res, vec![vec!["bat"],vec!["eat", "tea", "ate"], vec!["tan", "nat"]]);
}