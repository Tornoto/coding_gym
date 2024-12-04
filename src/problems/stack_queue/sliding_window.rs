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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let result = max_sliding_window_timeout(nums, 3);
        println!("res:{:?}", result);
    }
}
