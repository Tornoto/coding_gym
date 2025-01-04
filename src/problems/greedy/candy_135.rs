/// https://leetcode.com/problems/candy/description/
pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];

    // !!! 不要试图同时更新左右

    // 从前向后更新糖果数量
    // 如果右侧比左侧评分高, 则:
    // candies[idx] = candies[idx - 1] + 1;
    for idx in 1..ratings.len() {
        if ratings[idx] > ratings[idx - 1] {
            candies[idx] = candies[idx - 1] + 1;
        }
    }
    // 从后向前更新糖果数量
    // 如果左侧比右侧评分高, 则:
    // candies[idx] = candies[idx + 1] + 1;
    // 注意: 这一趟需要考虑 实际左侧糖果数量和右侧糖果数量的 大小关系
    // 经过第一趟，左侧 candies[idx] 可能已经大于 candies[idx + 1] + 1
    // 因此需要在 candies[idx] 和 candies[idx + 1] + 1 中取最大值
    // 从前向后不需要是因为初始默认都只给了 1 块糖。
    for idx in (0..ratings.len() - 1).rev() {
        if ratings[idx] > ratings[idx + 1] {
            candies[idx] = candies[idx].max(candies[idx + 1] + 1);
        }
    }

    candies.iter().sum::<i32>()
}

// 1 3 4 5 2
// 1 2 3  1

#[test]
fn test() {
    let ratings = vec![1, 3, 4, 5, 2];
    let candies = candy(ratings);
    println!("res: {candies}");
}
