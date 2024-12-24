use std::collections::HashMap;

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

/// https://leetcode.com/problems/subsets/description/
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut path = vec![];
    subsets_backtracing(&nums, &mut result, &mut path);
    result
}

fn subsets_backtracing(nums: &[i32], result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
    result.push(path.clone());

    for idx in 0..nums.len() {
        path.push(nums[idx]);
        subsets_backtracing(&nums[idx + 1..], result, path);
        path.pop();
    }
}

/// 迭代方法处理 subsets
pub fn subsets_iter(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];

    for &num in &nums {
        let mut sub_result = Vec::new();
        for subset in &result {
            let mut subset_copy = subset.clone();
            subset_copy.push(num);
            sub_result.push(subset_copy);
        }
        result.extend(sub_result);
    }

    result
}

/// https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/
pub fn letter_combinations(digits: String) -> Vec<String> {
    // 输入为空直接返回
    if digits.is_empty() {
        return vec![];
    }

    let map = HashMap::from([
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z']),
    ]);

    let mut result = vec![];

    let mut cur_combs = vec![String::new()];

    // 遍历输入的数字
    for ch_num in digits.chars() {
        // 获取数字对应的字符数组
        if let Some(letters) = map.get(&ch_num) {
            // 对于每个字符，复制之前的组合，
            // 并将字符插入组合中，生成新的组合
            // 然后将 当前组合 指向 新组合
            let mut new_combs = vec![];
            for letter in letters {
                for comb in &cur_combs {
                    let mut new_comb = comb.clone();
                    new_comb.push(*letter);
                    new_combs.push(new_comb);
                }
            }
            cur_combs = new_combs;
        }
    }
    result.extend(cur_combs);

    result
}

pub fn letter_combinations_bt(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let map = HashMap::from([
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['x', 'y', 'z']),
    ]);

    let mut result = vec![];
    let mut comb = String::new();
    letter_combinations_backtracing(&digits, digits.len(), &map, &mut result, &mut comb);
    result
}

fn letter_combinations_backtracing(
    digits: &str,
    size: usize,
    map: &HashMap<char, Vec<char>>,
    result: &mut Vec<String>,
    comb: &mut String,
) {
    if comb.len() == size {
        result.push(comb.clone());
    }

    if let Some(ch_num) = digits.chars().next() {
        if let Some(letters) = map.get(&ch_num) {
            for letter in letters {
                comb.push(*letter);
                letter_combinations_backtracing(&digits[1..], size, map, result, comb);
                comb.pop();
            }
        }
    }
}

/// https://leetcode.cn/problems/n-queens/description/
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];
    let mut layout: Vec<String> = vec![];
    n_queens_backtracing(n, 0, &mut result, &mut layout);
    result
}

/// 回溯
fn n_queens_backtracing(n: i32, row: i32, result: &mut Vec<Vec<String>>, layout: &mut Vec<String>) {
    if layout.len() == n as usize {
        result.push(layout.clone());
        return;
    }

    for col in 0..n {
        let queens = get_queen_pos(&layout);
        if can_put(&queens, row, col) {
            let line = put_queen(n, col);
            layout.push(line);
            n_queens_backtracing(n, row + 1, result, layout);
            layout.pop();
        }
    }
}

/// 放置皇后，生成”..Q..“样式字符串
fn put_queen(n: i32, pos: i32) -> String {
    let mut line = String::new();
    for _ in 0..pos {
        line.push('.');
    }
    line.push('Q');
    for _ in pos + 1..n {
        line.push('.');
    }

    line
}

/// 获取已放置皇后的坐标
fn get_queen_pos(layout: &Vec<String>) -> Vec<(i32, i32)> {
    if layout.len() == 0 {
        return vec![];
    }

    let mut pos = vec![];
    for row in 0..layout.len() {
        let line = &layout[row];
        let col = line.find("Q").unwrap();
        pos.push((row as i32, col as i32));
    }

    pos
}

/// 根据已有皇后位置，判断当前位置是否可以放置皇后
fn can_put(queens: &Vec<(i32, i32)>, row: i32, col: i32) -> bool {
    for queen in queens {
        // 不能同列
        if col == queen.1 {
            return false;
        }

        let row_diff = queen.0 - row;
        let col_diff = queen.1 - col;
        // 不能在同一条对角线上
        if row_diff == col_diff || row_diff + col_diff == 0 {
            return false;
        }
    }
    true
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

    #[test]
    fn test_subsets() {
        // 输入：nums = [1,2,3]
        // 输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
        let nums = vec![1, 2, 3];
        let result = subsets(nums);
        println!("{:?}", result);
        let nums = vec![0];
        let result = subsets(nums);
        println!("{:?}", result);
    }

    #[test]
    fn test_letter_combinations() {
        // Input: digits = "23"
        // Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
        let digits = "23";
        let result = letter_combinations_bt(digits.to_string());
        println!("{:?}", result);
    }

    #[test]
    fn test_solve_n_queens() {
        let result = solve_n_queens(4);
        println!("{:?}", result);
        // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
        // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
    }
}