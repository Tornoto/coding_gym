/// https://leetcode.cn/problems/dungeon-game/
pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let rows = dungeon.len();
    let cols = dungeon[0].len();

    let mut dp = vec![vec![0; cols]; rows];

    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            // 如果在终点
            if i == rows - 1 && j == cols - 1 {
                dp[i][j] = (1 - dungeon[i][j]).max(1);
            } else if i == rows - 1 {
                // 最后一行
                dp[i][j] = (dp[i][j + 1] - dungeon[i][j]).max(1);
            } else if j == cols - 1 {
                // 最后一列
                dp[i][j] = (dp[i + 1][j] - dungeon[i][j]).max(1);
            } else {
                dp[i][j] = (dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]).max(1);
            }
        }
    }

    dp[0][0]
}

#[test]
fn test() {
    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    calculate_minimum_hp(dungeon);
}
