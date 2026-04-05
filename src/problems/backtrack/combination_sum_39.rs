/// https://leetcode.com/problems/combination-sum/description/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut selection = vec![];

    helper(&candidates, target, &mut selection, &mut result);

    result
}

fn helper(candidates: &[i32], target: i32, selection: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let sum = selection.iter().sum::<i32>();
    // 如果凑够target则加入结果集，并返回
    if sum == target {
        result.push(selection.clone());
        return;
    }
    // 如果和超出target则返回
    if sum > target {
        return;
    }
    // 注意candidates中的元素可以重复选取
    // 因此递归调用时还是从i开始
    for i in 0..candidates.len() {
        selection.push(candidates[i]);
        helper(&candidates[i..], target, selection, result);
        selection.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combination_sum() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = combination_sum(candidates, target);
        println!("{:?}", result);

        let candidates = vec![2, 3, 5];
        let target = 8;
        let res = combination_sum(candidates, target);
        println!("{:?}", res);
    }
}
