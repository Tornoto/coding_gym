/// https://leetcode.com/problems/n-queens/description/
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut layout: Vec<i32> = vec![];
    n_queens_backtracing(n, 0, &mut result, &mut layout);
    let mut solutions = vec![];

    for layout in result {
        let mut sub_solution = vec![];
        for queen in layout {
            sub_solution.push(put_queen(n, queen));
        }
        solutions.push(sub_solution);
    }
    solutions
}

/// 回溯
fn n_queens_backtracing(n: i32, row: i32, result: &mut Vec<Vec<i32>>, layout: &mut Vec<i32>) {
    if layout.len() == n as usize {
        result.push(layout.clone());
        return;
    }

    for col in 0..n {
        if can_put(&layout, row, col) {
            layout.push(col);
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

/// 根据已有皇后位置，判断当前位置是否可以放置皇后
fn can_put(queens: &Vec<i32>, row: i32, col: i32) -> bool {
    for (qrow, qcol) in queens.iter().enumerate() {
        // 不能同列
        if *qcol == col {
            return false;
        }
        let row_diff = qrow as i32 - row;
        let col_diff = qcol - col;
        // 不能在同一条对角线上
        if row_diff == col_diff || row_diff + col_diff == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_n_queens() {
        let result = solve_n_queens(4);
        println!("{:?}", result);
        // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
        // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
    }
}
