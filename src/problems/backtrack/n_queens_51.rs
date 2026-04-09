use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/n-queens/description/
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn backtrack(n: i32, result: &mut Vec<Vec<i32>>, layout: &mut Vec<i32>) {
            if layout.len() as i32 == n {
                result.push(layout.clone());
                return;
            }

            let row = layout.len() as i32;

            for col in 0..n {
                if can_put(layout, row, col) {
                    layout.push(col as i32);
                    backtrack(n, result, layout);
                    layout.pop();
                }
            }
        }

        // 根据已有皇后位置，判断当前位置是否可以放置皇后
        fn can_put(layout: &[i32], row: i32, col: i32) -> bool {
            for (qrow, qcol) in layout.iter().enumerate() {
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

        // 放置皇后，生成”..Q..“样式字符串
        fn put_queen(n: i32, col: i32) -> String {
            (0..n).map(|c| if c == col { 'Q' } else { '.' }).collect()
        }

        let mut result = vec![];
        let mut layout = vec![];

        backtrack(n, &mut result, &mut layout);

        result
            .into_iter()
            .map(|layout| layout.iter().map(|&col| put_queen(n, col)).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_n_queens() {
        let result = Solution::solve_n_queens(5);
        println!("{:?}", result);
        // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
        // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
    }
}
