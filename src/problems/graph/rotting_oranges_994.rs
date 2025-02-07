use std::collections::VecDeque;

/// https://leetcode.com/problems/rotting-oranges/
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut grid = grid;
    let mut fresh_oranges = 0;
    let mut minutes = 0;

    let mut queue = VecDeque::new();

    // 遍历 计算新鲜橙子数量 将腐败的橙子入队列
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                fresh_oranges += 1;
            } else if grid[i][j] == 2 {
                queue.push_back((i, j));
            }
        }
    }

    // // 定义四个方向的偏移量
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while queue.is_empty() && fresh_oranges > 0 {
        minutes += 1;
        let size = queue.len();
        for _ in 0..size {
            let (row, col) = queue.pop_front().unwrap();
            for (dx, dy) in &dirs {
                let x = row as i32 + dx;
                let y = col as i32 + dy;
                if x < 0 || x >= rows as i32 || y < 0 || y >= cols as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if grid[x][y] == 1 {
                    grid[x][y] = 2;
                    fresh_oranges -= 1;
                    queue.push_back((x, y));
                }
            }
        }
    }

    if fresh_oranges == 0 {
        minutes
    } else {
        -1
    }
}
