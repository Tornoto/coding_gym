#![allow(unused)]
/// 给定一个背包，其容量为 C。有 n 个物品，每个物品重 w_i 价值 v_i
///
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

/// 使用一维数组 滚动更新
///
/// 观察原始的转移公式: `dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - w] + values[i - 1])`
///
/// 可以发现第i行实际上是在i-1行的基础上更新的，
///
/// 因此我们可以使用一个一维数组，每次都在原值的基础上进行更新。
///
/// `dp[j] = dp[j].max(dp[j-w] + values[i-1]`
///
/// 但更新的顺序有讲究，我们看到 `dp[j]`依赖于前面的 `dp[j-w]`，
///
/// 如果从左向右更新，必然会因为前面元素（左侧）的更新而影响后面元素（右侧）的更新
///
/// 我们可以从右向左更新，因为左侧的元素更新值不依赖于右侧的元素。
fn knapsack_1d(values: &[i32], weights: &[i32], capacity: i32) -> i32 {
    assert_eq!(values.len(), weights.len());
    let n = values.len();
    let capacity = capacity as usize;

    let mut dp = vec![0; capacity + 1];

    for i in 0..n {
        let w = weights[i] as usize;
        // 注意这里的范围是从右侧到 w，因为小于w的情形 dp[i][j] = dp[i-1][j]
        // 在一维数组中无需更新
        for j in (w..capacity + 1).rev() {
            dp[j] = dp[j].max(dp[j - w] + values[i]);
        }
    }

    dp[capacity]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let values = vec![120, 60, 100];
        let weights = vec![30, 10, 20];
        let capacity = 50;
        let max_value = knapsack_1d(&values, &weights, capacity);
        assert_eq!(max_value, 220);

        let values = vec![5, 1];
        let weights = vec![1, 2];
        let capacity = 2;
        let max_value = knapsack_1d(&values, &weights, capacity);
        assert_eq!(max_value, 5);
    }
}
