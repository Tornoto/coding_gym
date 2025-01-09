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
