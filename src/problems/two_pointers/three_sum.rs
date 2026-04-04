use crate::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut result = vec![];
        if len < 3 {
            return result;
        }

        let mut nums = nums;
        // must sort first
        nums.sort_unstable();

        for i in 0..len - 2 {
            // 如果第一个数大于0，后面肯定都大于0，直接结束
            if nums[i] > 0 {
                break;
            }
            // 跳过重复元素
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    // 跳过左侧重复元素
                    // 使用 left < right 防止越界
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    // 跳过右侧重复元素
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    // 移动指针寻找下一组解
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = Solution::three_sum(nums);
        println!("{:?}", res);
        // //
        let nums = vec![0, 1, 1];
        let res = Solution::three_sum(nums);
        println!("{:?}", res);

        let nums = vec![0, 0, 0];
        let res = Solution::three_sum(nums);
        println!("{:?}", res);

        let nums = vec![0, 0, 0, 0];
        let res = Solution::three_sum(nums);
        println!("{:?}", res);

        let nums = vec![1, -1, -1, 0];
        let res = Solution::three_sum(nums);
        println!("{:?}", res);
    }
}
