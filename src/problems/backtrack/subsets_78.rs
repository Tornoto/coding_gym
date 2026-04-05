/// https://leetcode.com/problems/subsets/description/
/// 递归法求解
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut selection = vec![];

    recursive_helper(&nums, &mut selection, &mut result);

    result
}

fn recursive_helper(nums: &[i32], selection: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(selection.clone());

    for (idx, num) in nums.iter().enumerate() {
        selection.push(*num);
        recursive_helper(&nums[idx + 1..], selection, result);
        selection.pop();
    }
}

/// 迭代法求解
pub fn subsets_iter(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    result.push(vec![]);

    for num in nums {
        let mut result_cloned = result.clone();
        for subset in result_cloned.iter_mut() {
            subset.push(num);
        }
        result.extend(result_cloned);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        // let result = subsets(nums);
        let result = subsets_iter(nums);
        println!("{:?}", result);
    }
}
