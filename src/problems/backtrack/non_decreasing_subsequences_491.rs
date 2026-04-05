#![allow(unused)]
use std::collections::HashSet;

pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut sequence = vec![];

    // use HashSet version
    // let mut result_set: HashSet<Vec<i32>> = HashSet::new();
    // backtrace(&nums, 0, &mut result_set, &mut sequence);
    // result.extend(result_set);

    // without HashSet version
    backtrace2(&nums, 0, i32::MIN, &mut result, &mut sequence);
    result
}

/// hashset version
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

/// without hashset
fn backtrace2(
    nums: &[i32],
    start: usize,
    last_picked: i32,
    result: &mut Vec<Vec<i32>>,
    sequence: &mut Vec<i32>,
) {
    // 如果到了末尾
    if start == nums.len() {
        if sequence.len() > 1 {
            result.push(sequence.clone());
        }
        return;
    }

    // If the current element can be picked
    // (is greater or equal to the last picked element)
    if nums[start] >= last_picked {
        sequence.push(nums[start]);
        backtrace2(nums, start + 1, nums[start], result, sequence);
        sequence.pop();
    }

    if nums[start] != last_picked {
        backtrace2(nums, start + 1, last_picked, result, sequence);
    }
}
