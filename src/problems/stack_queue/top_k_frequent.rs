use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut res: Vec<i32> = Vec::new();
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

    for val in nums {
        map.entry(val).and_modify(|e| *e += 1).or_insert(1);
    }

    for (num, frequency) in map.iter() {
        heap.push((*frequency, *num));
    }

    for _i in 0..k {
        res.push(heap.pop().unwrap().1);
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_top_k_frequency() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = top_k_frequent(nums, k);
        println!("res: {:?}", result);

        let nums = vec![1];
        let k = 1;
        let result = top_k_frequent(nums, k);
        println!("res: {:?}", result);
    }
}
