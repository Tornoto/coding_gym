#[allow(unused)]
/// 给定一个背包，其容量为 C。有 n 个物品，每个物品重 w_i 价值 v_i
/// 求解背包能容纳的最大价值
fn knapsack(values: &[i32], weights: &[i32], capacity: i32) -> i32 {
    assert_eq!(values.len(), weights.len());

    let n = values.len();
    let capacity = capacity as usize;
    // 定义 dp[i][j] 为在前 i 个物品中选择若干个物品放入容量为 j 的背包中所能获得的最大价值。
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..n + 1 {
        for j in 1..capacity + 1 {
            let w = weights[i - 1] as usize;
            // 如果第 i 个物品超过容量，则必然没法选取
            // 则前 i 个物品放入容量为 j 的背包的最大值 dp[i][j] 等于
            // 将前 i-1 个物品放入容量为 j 的背包的最大值 dp[i-1][j]
            if w > j {
                dp[i][j] = dp[i - 1][j];
            } else {
                // 若容量 j 大于等于当前物品体积，那么可供选择的方案有：
                // 1. 不采用当前物品 i
                // 2. 在前 i-1 个物品且容量为 j-w 的基础上，选择物品i
                // 取其中的大值
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - w] + values[i - 1]);
            }
        }
    }

    dp[n][capacity]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let values = vec![120, 60, 100];
        let weights = vec![30, 10, 20];
        let capacity = 50;
        let max_value = knapsack(&values, &weights, capacity);
        assert_eq!(max_value, 220);
    }
}
