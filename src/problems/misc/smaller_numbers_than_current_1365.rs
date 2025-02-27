/// https://leetcode.cn/problems/how-many-numbers-are-smaller-than-the-current-number/description/
///
/// 题目提示`nums[i]`数字范围为`[0,100]`, 因此可以用一个101长度的数组统计`nums[i]`出现的频率
/// 这样就省去了排序。然后从头向后遍历，统计比自己大的数字的个数。
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    // as 0 <= nums[i] <=100, the max len of count is 101
    let mut frequence = vec![0; 101];
    let mut count = vec![0; nums.len()];

    for &num in &nums {
        frequence[num as usize] += 1;
    }

    // 由于借助frequency数组 原地修改
    // 需要记住frequency中nums[i]之前的数字数量
    let mut smaller_num_counts = 0;
    for i in 0..frequence.len() {
        if frequence[i] > 0 {
            let tmp = smaller_num_counts;
            // 累加
            smaller_num_counts += frequence[i];
            frequence[i] = tmp;
        }
    }

    for i in 0..nums.len() {
        count[i] = frequence[nums[i] as usize];
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![8, 1, 2, 2, 3];
        let result = smaller_numbers_than_current(nums);
        assert_eq!(result, vec![4, 0, 1, 1, 3]);

        let nums = vec![6, 5, 4, 8];
        let result = smaller_numbers_than_current(nums);
        assert_eq!(result, vec![2, 1, 0, 3]);

        let nums = vec![7, 7, 7, 7];
        let result = smaller_numbers_than_current(nums);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }
}
