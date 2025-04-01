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
    // 以 [1,2,3,4] 为例，如果 k 为 2，则第一个数的遍历范围是 1,2,3
    let k_left = k - path.len() as i32;
    let end = n - k_left + 1;
    for idx in start_idx..=end {
        path.push(idx);
        combine_rec(n, k, idx + 1, result, path);
        path.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combine() {
        let res = combine(2, 2);
        println!("res: {:?}", res);
    }
}
