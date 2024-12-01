#[allow(unused)]
// https://leetcode.com/problems/remove-element/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut start: usize = 0;
    let mut end: usize = nums.len();

    // it's safe and clean to use start < end rather than start + 1 < end
    // for using start + 1 < end will not enter the loop while the array has only one ele.
    while start < end {
        // move end to the first element that does not equal target
        while end > 0 && nums[end - 1] == val {
            end -= 1;
        }

        if end < 1 {
            return start as i32;
        }
        // move start to the first element that equals target
        while start < nums.len() && nums[start] != val {
            start += 1;
        }

        if start + 1 < end {
            nums.swap(start, end - 1);
        }
    }

    start as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        println!("iteration 1:");
        let mut numbers = vec![1, 2, 3, 2, 3, 3, 4];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 2:");
        let mut numbers = vec![3, 3, 3, 3, 3, 3, 3];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 3:");
        let mut numbers = vec![3, 3, 3, 3, 3, 3, 3];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 10);
        println!("{:?}", numbers);

        println!("iteration 4:");
        let mut numbers = vec![1, 3, 3, 3, 3, 3, 3];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 5:");
        let mut numbers = vec![1, 3, 3, 3, 3, 3, 1, 1];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 6:");
        let mut numbers = vec![2];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);
    }
}
