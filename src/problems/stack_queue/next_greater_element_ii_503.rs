use std::collections::HashMap;

/// https://leetcode.com/problems/next-greater-element-ii/description/
/// 基于 next-greater-element-i，将原数组拼接，形成循环
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut nums2 = vec![];
    nums2.extend_from_slice(&nums);
    nums2.extend_from_slice(&nums);

    let n = nums2.len();
    let mut stack: Vec<(usize, i32)> = vec![];
    let mut map = HashMap::new();

    for i in 0..n {
        let val = nums2[i];
        while !stack.is_empty() && stack[stack.len() - 1].1 < val {
            let prev_dist = stack.pop().unwrap().0;
            map.insert(prev_dist, val);
        }
        stack.push((i, val));
    }

    let mut ans = vec![-1; nums.len()];
    for i in 0..nums.len() {
        if let Some(idx) = map.get(&i) {
            ans[i] = *idx;
        }
    }

    ans
}
