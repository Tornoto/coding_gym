use std::collections::HashMap;

use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/fruit-into-baskets/description/
    ///
    /// 使用滑动窗口 + HashMap 解决水果入篮问题
    /// 核心思路：维护一个最多包含 2 种水果的最长连续子数组
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        // HashMap 记录当前窗口内每种水果的数量
        let mut baskets: HashMap<i32, i32> = HashMap::new();
        let mut start = 0;
        let mut max_output = 0;

        for end in 0..fruits.len() {
            // 将当前水果加入篮子
            baskets
                .entry(fruits[end])
                .and_modify(|count| *count += 1)
                .or_insert(1);

            // 当篮子中水果种类超过 2 种时，收缩左边界
            while baskets.len() > 2 {
                // 左边界水果数量减 1
                baskets.entry(fruits[start]).and_modify(|count| *count -= 1);
                // 如果某种水果数量归零，从篮子中移除
                if *baskets.get(&fruits[start]).unwrap() == 0 {
                    baskets.remove(&fruits[start]);
                }
                start += 1;
            }

            // 此时窗口合法（最多 2 种水果），更新最大值
            max_output = max_output.max(end - start + 1);
        }

        max_output as i32
    }

    /// 滑动窗口状态机优化版
    pub fn total_fruit_improved(fruits: Vec<i32>) -> i32 {
        // 边界处理：长度 ≤2 时直接返回
        if fruits.len() <= 2 {
            return fruits.len() as i32;
        }

        let mut max_len = 0;

        // 篮子状态：记录窗口内两种水果的类型及数量
        let mut type1 = fruits[0];
        let mut count1 = 1;
        // 使用 i32::MIN 作为未初始化哨兵，兼容任意 i32 输入且无额外分支开销
        let mut type2 = i32::MIN;
        let mut count2 = 0;

        // 滑动窗口辅助状态：记录末尾连续相同水果的类型和长度
        let mut last_type = fruits[0];
        let mut last_consecutive = 1;

        for &f in fruits.iter().skip(1) {
            if f == type1 {
                count1 += 1;
            } else if f == type2 {
                count2 += 1;
            } else {
                if last_type == type1 {
                    type2 = f;
                    count2 = 1;
                    count1 = last_consecutive;
                } else {
                    type1 = f;
                    count1 = 1;
                    count2 = last_consecutive;
                }
            }
            // 更新历史最大值
            max_len = max_len.max(count1 + count2);

            // 更新末尾连续计数状态
            if f == last_type {
                last_consecutive += 1;
            } else {
                last_type = f;
                last_consecutive = 1;
            }
        }

        max_len
    }
}

#[test]
fn test() {
    let fruits = vec![1, 2, 1, 3, 3, 3, 3, 3];
    let total = Solution::total_fruit(fruits);
    println!("total: {total}");
}
