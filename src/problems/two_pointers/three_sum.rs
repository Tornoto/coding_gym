pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }

    let mut nums = nums;
    nums.sort();

    let mut res: Vec<Vec<i32>> = vec![];

    let mut start: usize = 0;

    // prune: if nums[0] > 0 means no solution
    if nums[start] > 0 {
        return res;
    }

    while start < nums.len() - 2 {
        // skip same number for start, pay attention to condition: start > 0
        // without this condition, solution like [0,0,0] will be ignored
        // aslo do not need to use a inner while to skip,
        // use if continue, and the outer while will help you skip
        if start > 0 && nums[start] == nums[start - 1] {
            start += 1;
            continue;
        }
        let mut p1: usize = start + 1;
        let mut p2: usize = nums.len();
        while p1 < p2 - 1 {
            let sum = nums[start] + nums[p1] + nums[p2 - 1];
            // if sum != 0, there is no need to skip same number for p1 or p2
            // as next loop will find sum != 0 and will move p1 or p2
            if sum > 0 {
                p2 -= 1;
            } else if sum < 0 {
                p1 += 1;
            } else {
                let sub_res = vec![nums[start], nums[p1], nums[p2 - 1]];
                res.push(sub_res);
                while p2 > 2 && nums[p2 - 2] == nums[p2 - 1] {
                    p2 -= 1;
                }
                p2 -= 1;
                while p1 < nums.len() - 1 && nums[p1 + 1] == nums[p1] {
                    p1 += 1;
                }
                p1 += 1;
            }
        }
        start += 1;
    }

    res
}

pub fn three_sum2(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }
    let mut nums = nums;
    nums.sort_unstable();

    let mut start = 0;

    let mut result = vec![];
    while start < nums.len() - 2 {
        // 提前退出
        if nums[start] > 0 {
            return result;
        }
        let mut left = start + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[start] + nums[left] + nums[right];
            if sum == 0 {
                result.push(vec![nums[start], nums[left], nums[right]]);

                while right > 1 && nums[right - 1] == nums[right] {
                    right -= 1;
                }
                right -= 1;

                while left < nums.len() - 1 && nums[left + 1] == nums[left] {
                    left += 1;
                }
                left += 1;
            } else if sum > 0 {
                // 没必要跳过重复的元素。因为即使不跳过，下次循环进来发现sum>0,也会继续移动
                // 这和提前移动去除重复元素的效率是一样的
                right -= 1;
            } else {
                left += 1;
            }
        }
        // the start pointer moves right and skips elements of the same value
        while start < nums.len() - 2 && nums[start + 1] == nums[start] {
            start += 1;
        }
        start += 1;
    }

    result
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = three_sum2(nums);
        println!("{:?}", res);
        // //
        let nums = vec![0, 1, 1];
        let res = three_sum2(nums);
        println!("{:?}", res);

        let nums = vec![0, 0, 0];
        let res = three_sum2(nums);
        println!("{:?}", res);

        let nums = vec![0, 0, 0, 0];
        let res = three_sum2(nums);
        println!("{:?}", res);
    }
}
