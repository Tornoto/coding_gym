use std::collections::HashSet;

/// https://leetcode.com/problems/happy-number/description/
pub fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut set = HashSet::new();
    loop {
        let sum = calculate(n);
        if sum == 1 {
            return true;
        } else {
            if set.contains(&sum) {
                return false;
            } else {
                set.insert(sum);
                n = sum;
            }
        }
    }
}

fn calculate(n: i32) -> i32 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        let remainder = n % 10;
        sum += remainder * remainder;
        n /= 10;
    }

    sum
}
