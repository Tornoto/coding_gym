use std::collections::HashSet;

pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut sequence = vec![];
    let mut result_set: HashSet<Vec<i32>> = HashSet::new();

    backtrace(&nums, 0, &mut result_set, &mut sequence);

    result.extend(result_set);
    result
}

fn backtrace(
    nums: &[i32],
    start: usize,
    result_set: &mut HashSet<Vec<i32>>,
    sequence: &mut Vec<i32>,
) {
    if sequence.len() > 1 {
        result_set.insert(sequence.clone());
    }

    for (index, value) in nums.iter().enumerate() {
        if index < start {
            continue;
        }

        if index > start && *value == nums[index - 1] {
            continue;
        }

        // 如果当前 sequence 为空 或者 最后的元素不大于当前元素
        if let Some(last) = sequence.last() {
            if *last <= *value {
                sequence.push(*value);
                backtrace(nums, index + 1, result_set, sequence);
                sequence.pop();
            }
        } else {
            sequence.push(*value);
            backtrace(nums, index + 1, result_set, sequence);
            sequence.pop();
        }
    }
}
