/// https://leetcode.com/problems/integer-break/description/
/// 尽可能将 n 拆成 3
/// 如果余 1, 则 拆成 3, 3, ..., 4
/// 如果余 2, 则 拆成 3, 3, ..., 5
pub fn integer_break_greedy(n: i32) -> i32 {
    if n == 2 {
        return 1;
    }

    if n == 3 {
        return 2;
    }

    if n == 4 {
        return 4;
    }

    let base: i32 = 3;
    if n % 3 == 0 {
        let exponent = (n / 3) as u32;
        return base.pow(exponent);
    } else if n % 3 == 1 {
        let exponent = (n / 3 - 1) as u32;
        return base.pow(exponent) * 4;
    } else {
        let exponent = (n / 3 - 1) as u32;
        return base.pow(exponent) * 6;
    }
}

/// 动态规划求解
///
/// `dp[i]` 取值的几种情况:
/// 1. 遍历 j, `dp[i] = j * dp[i-j]`
/// 2. 遍历 j, `dp[i] = j * (i-j)`
/// 3. 由于遍历 j, `dp[i]` 会在遍历过程中更新，因此需要取遍历过程中最大的 `dp[i]`
///
/// j 的遍历范围如何确定？
///
/// 我们观察这个序列发现，当 `n > 2` 时，`n = 2 * a + 3 * b, a ≥ 0, b ≥ 0`
///
/// 4 = 2 + 2 且 2 * 2 是和为 4 时的最大乘积。
///
/// 因此我们只要遍历 j 为 2 和 3 的情形
pub fn integer_break_dp(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    // 初始化 d[2] = 1 版本
    dp[2] = 1;
    for i in 3..n + 1 {
        for j in 2..=3 {
            if i <= j {
                break;
            }
            let multi1 = j * (i - j);
            let multi2 = j * dp[i - j];
            dp[i] = dp[i].max(multi1.max(multi2));
        }
    }

    // 不初始化 dp[2] 版本
    // for i in 2..n + 1 {
    //     for j in 1..=3 {
    //         if i <= j {
    //             break;
    //         }
    //         let multi1 = j * (i - j);
    //         let multi2 = j * dp[i - j];
    //         dp[i] = dp[i].max(multi1.max(multi2));
    //     }
    // }

    dp[n] as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        for n in 2..20 {
            // let n = 5;
            let v1 = integer_break_greedy(n);
            let v2 = integer_break_dp(n);
            assert_eq!(v1, v2);
            println!("{:?}: {:?}, {:?}", n, v1, v2);
        }
    }
}
