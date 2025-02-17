/// https://leetcode.com/problems/factorial-trailing-zeroes/
pub fn trailing_zeroes(n: i32) -> i32 {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        count += n / 5;
        n /= 5;
    }
    count
}
