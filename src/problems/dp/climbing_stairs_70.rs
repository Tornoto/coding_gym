/// https://leetcode.com/problems/climbing-stairs/description/
pub fn climb_stairs(n: i32) -> i32 {
    let n: usize = n as usize;
    if n == 1 {
        return 1;
    }

    let mut ways = vec![0; n + 1];
    ways[1] = 1;
    ways[2] = 2;

    for i in 3..n + 1 {
        ways[i] = ways[i - 1] + ways[i - 2];
    }

    ways[n]
}

pub fn climb_stairs_v2(n: i32) -> i32 {
    let n = n as usize;
    if n < 2 {
        return 1;
    }

    let mut ways1 = 1;
    let mut ways2 = 2;

    for _ in 3..n + 1 {
        let ways = ways1 + ways2;
        ways1 = ways2;
        ways2 = ways;
    }

    ways2
}
