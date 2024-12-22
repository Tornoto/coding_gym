/// https://leetcode.com/problems/combinations/description/
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut path = vec![];
    combine_rec(n, k, 1, &mut result, &mut path);
    result
}

fn combine_rec(n: i32, k: i32, start_idx: i32, result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
    if path.len() == k as usize {
        result.push(path.clone());
        return;
    }

    // 剪枝
    let k_left = k - path.len() as i32;
    let end = n - k_left + 1;
    for idx in start_idx..=end {
        path.push(idx);
        combine_rec(n, k, idx + 1, result, path);
        path.pop();
    }
}

/// https://leetcode.com/problems/combination-sum-iii/description/
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut path = vec![];
    // 数字范围 1-9
    combination_sum3_rec(k, n, 1, 9, &mut result, &mut path);
    result
}

fn combination_sum3_rec(
    k: i32,
    n: i32,
    start_idx: i32,
    end_idx_inclusive: i32,
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
) {
    if path.len() == k as usize {
        if n == path.iter().sum() {
            result.push(path.clone());
        }
        return;
    }

    let k_left = k - path.len() as i32;
    // 写死9
    let end = 9 - k_left + 1;
    for idx in start_idx..=end {
        path.push(idx);
        combination_sum3_rec(k, n, idx + 1, end_idx_inclusive, result, path);
        path.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combine() {
        let res = combine(4, 2);
        println!("res: {:?}", res);
    }

    #[test]
    fn test_combination_sum3() {
        // 输入: k = 3, n = 7
        // 输出: [[1,2,4]]
        let result = combination_sum3(3, 7);
        println!("res: {:?}", result);

        // 输入: k = 3, n = 9
        // 输出: [[1,2,6], [1,3,5], [2,3,4]]
        let result = combination_sum3(3, 9);
        println!("res: {:?}", result);
    }
}
