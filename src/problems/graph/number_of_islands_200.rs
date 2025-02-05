/// https://leetcode.com/problems/number-of-islands/
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;

    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&mut grid, i, j);
            }
        }
    }

    count
}

fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let m = grid.len();
    let n = grid[0].len();

    if i >= m || j >= n || grid[i][j] == '0' {
        return;
    }
    // 标记为已访问
    grid[i][j] = '0';

    // 遍历周边
    if i > 0 {
        dfs(grid, i - 1, j);
    }
    if j + 1 < n {
        dfs(grid, i, j + 1);
    }
    if i + 1 < m {
        dfs(grid, i + 1, j);
    }
    if j > 0 {
        dfs(grid, i, j - 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let count = num_islands(grid);
        assert_eq!(count, 3);
    }
}
