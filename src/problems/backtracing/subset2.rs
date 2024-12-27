pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![];
    }

    let mut nums = nums;
    // sort!
    nums.sort();

    let mut result = vec![];
    let mut path = vec![];
    subsets2_backtracing(&nums, 0, &mut result, &mut path);

    result
}

fn subsets2_backtracing(
    nums: &[i32],
    start: usize,
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
) {
    result.push(path.clone());
    for idx in start..nums.len() {
        // skip when same with previous value
        if idx > start && nums[idx] == nums[idx - 1] {
            continue;
        }
        path.push(nums[idx]);
        subsets2_backtracing(nums, idx + 1, result, path);
        path.pop();
    }
}
