use std::collections::HashMap;

/// https://leetcode.com/problems/next-greater-element-i/description/
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let n = nums2.len();
    let mut stack = vec![];
    // 存放nums2中元素和其右侧第一个大值的映射关系
    // key:ele, value: next greater ele
    let mut map = HashMap::new();

    // 使用栈，统计nums2中，每个元素右侧第一个 geater element
    for i in 0..n {
        let val = nums2[i];
        // 如果栈内元素小于即将入栈的元素，则更新map
        while !stack.is_empty() && stack[stack.len() - 1] < val {
            let prev_val = stack[stack.len() - 1];
            map.insert(prev_val, val);
        }
        // 元素入栈
        stack.push(val);
    }

    let mut ans = vec![-1; nums1.len()];
    // 根据nums1的元素，找到元素值对应的下一个大值
    for i in 0..nums1.len() {
        if let Some(idx) = map.get(&nums1[i]) {
            ans[i] = *idx;
        }
    }

    ans
}
