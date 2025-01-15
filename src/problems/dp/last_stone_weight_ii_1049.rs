/// https://leetcode.com/problems/last-stone-weight-ii/description/
/// 与等和子集问题类似，等和子集是让背包内容是否达到sum/2
/// 而这个则尽可能让背包内容达到sum/2
/// 最终结果就是 sum - 2*实际达到容量
pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum = stones.iter().sum::<i32>();
    let capacity = (sum / 2) as usize;

    let mut dp = vec![0; capacity + 1];

    for stone in stones {
        for j in (stone as usize..capacity + 1).rev() {
            dp[j] = dp[j].max(dp[j - stone as usize] + stone)
        }
    }

    sum - 2 * dp[capacity]
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        let result = last_stone_weight_ii(stones);
        println!("{:?}", result);

        let stones = vec![31, 26, 33, 21, 40];
        let result = last_stone_weight_ii(stones);
        println!("{:?}", result);

        let stones = vec![2, 2];
        let result = last_stone_weight_ii(stones);
        println!("{:?}", result);
    }
}
