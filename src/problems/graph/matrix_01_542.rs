use std::collections::VecDeque;

/// https://leetcode.com/problems/01-matrix/description/
pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if mat.len() == 0 || mat[0].len() == 0 {
        return mat;
    }
    let rows = mat.len();
    let cols = mat[0].len();
    let mut dist = vec![vec![0; cols]; rows];
    let mut queue = VecDeque::new();

    // 初始化 dist; 将值为0的坐标入队列
    for i in 0..rows {
        for j in 0..cols {
            if mat[i][j] == 0 {
                queue.push_back((i, j));
            } else {
                dist[i][j] = i32::MAX;
            }
        }
    }

    // 定义四个方向的偏移量
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((row, col)) = queue.pop_front() {
        for (dx, dy) in &dirs {
            let x = row as i32 + dx;
            let y = col as i32 + dy;
            if x >= 0 && x < rows as i32 && y >= 0 && y < cols as i32 {
                let x = x as usize;
                let y = y as usize;
                if dist[x][y] > dist[row][col] + 1 {
                    dist[x][y] = dist[row][col] + 1;
                    queue.push_back((x, y));
                }
            }
        }
    }

    dist
}
