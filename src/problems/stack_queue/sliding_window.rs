#![allow(unused)]

use std::collections::VecDeque;

/// https://leetcode.com/problems/sliding-window-maximum/
/// Time Limit Exceeded version
/// 37 / 51 testcases passed
pub fn max_sliding_window_timeout(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        let cur = nums[i];
        if i < (nums.len() - k as usize + 1) {
            result.push(cur);
        }
        // 更新能影响到的范围
        let j = i;
        let mut backward_count: usize = k as usize - 1;
        while backward_count > 0 {
            if j >= backward_count {
                let idx = j - backward_count;
                if idx < result.len() {
                    if result[j - backward_count] < cur {
                        result[j - backward_count] = cur;
                    }
                }
            }
            backward_count -= 1;
        }
    }

    result
}

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut deque = VecDeque::<usize>::new();
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());

    for i in 0..nums.len() {
        // remove element's idx out of the window
        if let Some(idx) = deque.front() {
            // *idx <= i -k
            if *idx + k < i + 1 {
                deque.pop_front();
            }
        }

        // remove idx of element which smaller than cur element
        while let Some(val) = deque.back() {
            if nums[*val] < nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        // push index of cur element into deque
        deque.push_back(i);

        // update the result
        if i >= k - 1 {
            if let Some(idx) = deque.front() {
                result.push(nums[*idx]);
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let result = max_sliding_window_timeout(nums, 3);
        println!("res:{:?}", result);

        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let result = max_sliding_window(nums, 3);
        println!("res:{:?}", result);

        let nums = vec![1, 3, 1, 2, 0, 5];
        let result = max_sliding_window(nums, 3);
        println!("res:{:?}", result);
    }
}
