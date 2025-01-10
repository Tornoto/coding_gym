/// https://leetcode.com/problems/unique-paths-ii/
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut board = vec![vec![0; n]; m];

    // 初始化首行
    for i in 0..n {
        if obstacle_grid[0][i] == 1 {
            break;
        }
        board[0][i] = 1;
    }
    // 初始化首列
    for i in 0..m {
        if obstacle_grid[i][0] == 1 {
            break;
        }
        board[i][0] = 1;
    }

    // 动态规划
    for i in 1..m {
        for j in 1..n {
            // 如果不可达，置路径数为 0
            if obstacle_grid[i][j] == 1 {
                board[i][j] = 0;
            } else {
                board[i][j] = board[i - 1][j] + board[i][j - 1];
            }
        }
    }

    board[m - 1][n - 1]
}
