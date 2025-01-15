use std::collections::BinaryHeap;

/// https://leetcode.com/problems/last-stone-weight/description/
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);

    while heap.len() > 1 {
        let stone1 = heap.pop().unwrap();
        let stone2 = heap.pop().unwrap();
        if stone1 != stone2 {
            if stone1 > stone2 {
                heap.push(stone1 - stone2);
            } else {
                heap.push(stone2 - stone1);
            }
        } else {
            heap.push(0);
        }
    }

    heap.pop().unwrap()
}
