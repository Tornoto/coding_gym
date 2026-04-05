/// https://leetcode.com/problems/combination-sum-ii/description/
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut result = vec![];
    let mut selection = vec![];

    helper(&candidates, target, &mut selection, &mut result);

    result
}

fn helper(candidates: &[i32], target: i32, selection: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let sum: i32 = selection.iter().sum();

    if sum == target {
        result.push(selection.clone());
        return;
    }

    if sum > target {
        return;
    }

    for (idx, val) in candidates.iter().enumerate() {
        // 跳过重复项
        if idx > 0 && candidates[idx] == candidates[idx - 1] {
            continue;
        }
        // 提前退出
        if sum + *val > target {
            break;
        }
        selection.push(*val);
        helper(&candidates[idx + 1..], target, selection, result);
        selection.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combination_sum2() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = combination_sum2(candidates, target);
        println!(">> {:?}", result);
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let result = combination_sum2(candidates, target);
        println!(">> {:?}", result);
    }
}
