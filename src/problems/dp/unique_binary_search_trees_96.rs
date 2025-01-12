/// https://leetcode.cn/problems/unique-binary-search-trees/description/
///
/// 定义 `dp[i]` 为 i 个节点可能的搜索二叉树数量
///
/// 观察发现 `dp[1] = 1`; 额外定义 `dp[0] = 1`
///
/// 以 `dp[3]` 为例, 每列的含义为: 左侧节点数 根节点值 右侧节点数 二叉搜索树数量
///
/// 最后一列的值 = 左侧二叉搜索树数量 * 右侧的二叉搜索树数量, 即`dp[l] * dp[r]`
/// ```
/// dp[3]
/// 0 1 2 ？
/// 1 2 1 1
/// 2 3 0 ？
/// dp[3] = dp[0] * dp[2] + dp[1] * dp[1] + dp[2] * dp[0]
/// 我们需要计算 dp[2]
/// 0 1 1 1
/// 1 2 0 1
/// dp[2] = dp[0] * dp[1] + dp[1] * dp[0] = 1 + 1 = 2
/// 因此 dp[3] = 2 + 1 + 2 = 5
/// ```
///
/// 因此我们可以从 2 向后推导。根据上面的公式我们可以发现公式:
/// ```
/// dp[i] = dp[0] * dp[i-1] + dp[1] * dp[i-2] + ... + dp[i-1] * dp[0]
/// ```
pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..n + 1 {
        for j in 0..i {
            dp[i] += dp[j] * dp[i - 1 - j];
        }
    }
    dp[n]
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let count = num_trees(n);
        assert_eq!(count, 5);

        let n = 5;
        let count = num_trees(n);
        assert_eq!(count, 42);
    }
}
