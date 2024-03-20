use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (idx, ele) in nums.iter().enumerate() {
        let other = target - ele;
        if let Some(other_idx) = map.get(&other) {
            return vec![*other_idx as i32, idx as i32];
        } else {
            map.insert(*ele, idx as i32);
        }
    }
    Vec::new()
}

#[test]

fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}