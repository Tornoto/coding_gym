/// https://leetcode.com/problems/fibonacci-number/
pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut seq = vec![0; n as usize + 1];
    seq[0] = 0;
    seq[1] = 1;

    for i in 2..n as usize + 1 {
        seq[i] = seq[i - 1] + seq[i - 2];
    }

    seq[n as usize]
}

pub fn fib_v2(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut f0 = 0;
    let mut f1 = 1;

    for _ in 2..n + 1 {
        let next = f0 + f1;
        f0 = f1;
        f1 = next;
    }

    f1
}
