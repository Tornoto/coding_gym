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
