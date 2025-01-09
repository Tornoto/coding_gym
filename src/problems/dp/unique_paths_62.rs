/// https://leetcode.com/problems/unique-paths/description/
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut board = vec![vec![0; n]; m];

    for i in 0..n {
        board[0][i] = 1;
    }

    for i in 0..m {
        board[i][0] = 1;
    }

    for i in 1..m {
        for j in 1..n {
            board[i][j] = board[i - 1][j] + board[i][j - 1];
        }
    }

    board[m - 1][n - 1]
}

pub fn unique_paths_v2(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut paths = vec![1; n];

    for _ in 1..m {
        for i in 1..n {
            paths[i] += paths[i - 1];
        }
    }

    paths[n - 1]
}
