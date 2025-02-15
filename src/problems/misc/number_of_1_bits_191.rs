///https://leetcode.com/problems/number-of-1-bits/
pub fn hamming_weight(n: i32) -> i32 {
    let mut n = n;
    let mut count = 0;

    while n > 0 {
        if n % 2 == 1 {
            count += 1;
        }
        n >>= 1;
    }

    count
}
