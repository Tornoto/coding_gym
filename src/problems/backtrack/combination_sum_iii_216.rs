/// https://leetcode.com/problems/combination-sum-iii/description/
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut selection = vec![];

    helper(k, n, 1, &mut selection, &mut result);

    result
}

fn helper(k: i32, n: i32, start_idx: i32, selection: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if selection.len() == 3 {
        let sum: i32 = selection.iter().sum();
        if sum == n {
            result.push(selection.clone());
        }
        return;
    }

    // 注意范围 只能选取1-9的数组
    let k_left = k - selection.len() as i32;
    let end = 9 - k_left + 1;
    for i in start_idx..=end {
        selection.push(i);
        helper(k, n, i + 1, selection, result);
        selection.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
