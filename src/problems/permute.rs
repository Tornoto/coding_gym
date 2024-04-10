pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let len = nums.len();
    backtrack(&mut res, &mut nums, 0, len);
    res
}

pub fn backtrack(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, first: usize, len: usize) {
    // 如果所有的都填完了
    if (first == len) {
        res.push(nums.clone());
        return;
    }

    for i in first..len {
        // 动态维护数组
        swap(nums, i, first);
        backtrack(res, nums, first + 1, len);
        swap(nums, i, first);
    }
}

pub fn swap(nums: &mut Vec<i32>, pos1: usize, pos2: usize) {
    let tmp = nums[pos1];
    nums[pos1] = nums[pos2];
    nums[pos2] = tmp;
}