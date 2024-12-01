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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = three_sum(nums);
        println!("{:?}", res);
        // //
        let nums = vec![0, 1, 1];
        let res = three_sum(nums);
        println!("{:?}", res);

        let nums = vec![0, 0, 0];
        let res = three_sum(nums);
        println!("{:?}", res);

        let nums = vec![0, 0, 0, 0];
        let res = three_sum(nums);
        println!("{:?}", res);
    }
}
