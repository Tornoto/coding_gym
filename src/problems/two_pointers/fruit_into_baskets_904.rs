use std::collections::HashMap;

/// https://leetcode.com/problems/fruit-into-baskets/description/

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut fruit_type_map: HashMap<i32, i32> = HashMap::new();

    let n = fruits.len();
    let mut start = 0;
    let mut end = 0;
    let mut max_output = 0;
    while end < n {
        fruit_type_map
            .entry(fruits[end])
            .and_modify(|count| *count += 1)
            .or_insert(1);

        // 如果加入 end 对应种类的水果后，种类大于两种，
        // 则应该调整 start 使种类保持为两种
        while fruit_type_map.len() > 2 {
            fruit_type_map
                .entry(fruits[start])
                .and_modify(|count| *count -= 1);
            if *fruit_type_map.get(&fruits[start]).unwrap() == 0 {
                fruit_type_map.remove(&fruits[start]);
            }
            start += 1;
        }

        // 如果种类小于等于两种，则更新ouput
        if fruit_type_map.len() <= 2 {
            max_output = max_output.max(end - start + 1);
            end += 1;
        }
    }
    max_output as i32
}

#[test]
fn test() {
    let fruits = vec![1, 2, 1, 3, 3, 3, 3, 3];
    let total = total_fruit(fruits);
    println!("total: {total}");
}
