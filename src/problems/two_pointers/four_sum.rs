/// https://leetcode.com/problems/4sum/
/// pay attention to numberic overflow
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();

    let mut res: Vec<Vec<i32>> = vec![];
    for i in 0..nums.len() - 3 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..nums.len() - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            let expected = target as i64 - nums[i] as i64 - nums[j] as i64;
            let mut start = j + 1;
            let mut end = nums.len();
            while start < end - 1 {
                if expected == nums[start] as i64 + nums[end - 1] as i64 {
                    res.push(vec![nums[i], nums[j], nums[start], nums[end - 1]]);
                    // skip
                    while start < nums.len() - 1 && nums[start + 1] == nums[start] {
                        start += 1;
                    }
                    while end > 3 && nums[end - 2] == nums[end - 1] {
                        end -= 1;
                    }
                    start += 1;
                    end -= 1;
                } else if expected > nums[start] as i64 + nums[end - 1] as i64 {
                    start += 1;
                } else {
                    end -= 1;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_four_sum() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let res = four_sum(nums, 0);
        println!("result: {:?}", res);

        let nums = vec![2, 2, 2, 2, 2];
        let res = four_sum(nums, 8);
        println!("result: {:?}", res);

        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let res = four_sum(nums, -294967296);
        println!("result: {:?}", res);
    }
}
