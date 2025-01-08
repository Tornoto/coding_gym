/// https://leetcode.com/problems/monotone-increasing-digits/description/
pub fn monotone_increasing_digits(n: i32) -> i32 {
    if n < 10 {
        return n;
    }

    let mut divisor = 1;
    let mut target_div = 1;
    let mut last_pick = -1;

    while divisor <= n {
        let low = (n / divisor) % 10;
        let hight = (n / divisor / 10) % 10;
        // last_pick 不为 -1 说明发现高位小于低位
        // 如果高位和 last_pick 相等，说明高位重复
        if hight == last_pick {
            divisor *= 10;
            target_div *= 10;
            continue;
        } else {
            // 一旦高位不重复，则将 last_pick 置为 -1
            last_pick = -1;
        }
        if low < hight {
            target_div = divisor * 10;
            // 为了处理 1332 这种带重复的
            // 如果不跳过重复的，会出现结果为 1329 这个错误答案
            // 而正确答案应该是 1299
            last_pick = hight;
        }
        divisor *= 10;
    }

    if target_div > 1 {
        return (n / target_div / 10) * target_div * 10
            + (((n / target_div) % 10) - 1) * target_div
            + (target_div - 1);
    }

    return n;
}

#[allow(unused)]
fn monotone_increasing_digits_v2(n: i32) -> i32 {
    if n < 10 {
        return n;
    }

    let mut digits: Vec<i32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut i = 1;
    while i < digits.len() && digits[i - 1] <= digits[i] {
        i += 1;
    }

    if i < digits.len() {
        // 当前 i 的位置是首个小于高位的数字
        // 向前移动，跳过和前面高位相等的元素
        // 13332 - 13322 - 13222 - 12222 - 12999
        while i > 0 && digits[i - 1] > digits[i] {
            digits[i - 1] -= 1;
            i -= 1;
        }
        // 补 9
        for j in i + 1..digits.len() {
            digits[j] = 9;
        }
    }

    let mut result = 0;
    for &digit in &digits {
        result = result * 10 + digit;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let n = 10;
        let res = monotone_increasing_digits_v2(n);
        assert_eq!(res, 9);

        let n = 1234;
        let res = monotone_increasing_digits_v2(n);
        assert_eq!(res, 1234);

        let n = 1332;
        let res = monotone_increasing_digits_v2(n);
        assert_eq!(res, 1299);
    }
}
