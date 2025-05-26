use std::collections::HashMap;

/// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut expect = HashMap::new();
    for i in 0..nums.len() {
        if expect.contains_key(&nums[i]) {
            return vec![*expect.get(&nums[i]).unwrap(), i as i32];
        } else {
            expect.insert(target - nums[i], i as i32);
        }
    }
    vec![]
}
