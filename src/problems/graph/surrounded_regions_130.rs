use std::collections::VecDeque;

/// https://leetcode.com/problems/surrounded-regions/
///
/// 这道题和第200题岛屿数量相似，区别在于如何区分完全包围和未完全包围的区域
///
/// 观察未被包围的区域，可以发现，这种区域必然和边缘相连。
///
/// 可以分两步走，
/// 1. 遍历边缘，将和边缘相连的'O'区域标记为'P'
/// 2. 如果单元格是 'O'，说明它是被包围的，将其替换为 'X'; 如果单元格是 'P'，说明它是与边界相连的，将其恢复为 'O'。
pub fn solve(board: &mut Vec<Vec<char>>) {
    if board.is_empty() || board[0].is_empty() {
        return;
    }
    let m = board.len();
    let n = board[0].len();

    // 遍历上下左右边框，标记与边界相连的'O'为'P'
    for i in 0..m {
        if board[i][0] == 'O' {
            bfs(board, i, 0);
        }
        if board[i][n - 1] == 'O' {
            bfs(board, i, n - 1);
        }
    }
    for j in 0..n {
        if board[0][j] == 'O' {
            bfs(board, 0, j);
        }
        if board[m - 1][j] == 'O' {
            bfs(board, m - 1, j);
        }
    }

    // 遍历整体，如果是'O'则替换成'X' 如果是'P'则替换成'O'
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'O' {
                board[i][j] = 'X';
            } else if board[i][j] == 'P' {
                board[i][j] = 'O';
            }
        }
    }
}

fn bfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let m = board.len();
    let n = board[0].len();
    let mut queue = VecDeque::new();
    queue.push_back((i, j));

    while let Some((i, j)) = queue.pop_front() {
        if i >= m || j >= n || board[i][j] != 'O' {
            continue;
        }
        board[i][j] = 'P';
        // up
        if i > 0 {
            queue.push_back((i - 1, j));
        }
        // right
        if j + 1 < n {
            queue.push_back((i, j + 1));
        }
        // down
        if i + 1 < m {
            queue.push_back((i + 1, j));
        }
        // left
        if j > 0 {
            queue.push_back((i, j - 1));
        }
    }
}
