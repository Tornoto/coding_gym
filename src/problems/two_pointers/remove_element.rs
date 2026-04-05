use crate::Solution;

#[allow(unused)]

impl Solution {
    /// This solution employs the fast-slow pointer technique to efficiently solve the problem. In this approach:
    ///
    /// - The fast pointer traverses the vector one step at a time, examining each element sequentially.
    ///
    /// - The slow pointer, on the other hand, only advances when the element pointed to by the fast pointer does not match the target value.
    ///
    /// By using this method, we ensure that all instances of the target value are effectively skipped, and only the desired elements are retained in the vector. This results in an in-place modification of the vector, optimizing both time and space complexity.
    ///
    /// https://leetcode.com/problems/remove-element/
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[slow] = nums[i];
                slow += 1;
            }
        }

        slow as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        println!("iteration 1:");
        let mut numbers = vec![1, 2, 3, 2, 3, 3, 4];
        Solution::remove_element(&mut numbers, 3);
        assert_eq!(numbers, vec![1, 2, 2, 4, 3, 3, 4]);

        println!("iteration 2:");
        let mut numbers = vec![3, 3, 3, 3, 3, 3, 3];
        Solution::remove_element(&mut numbers, 3);
        assert_eq!(numbers, vec![3, 3, 3, 3, 3, 3, 3]);

        println!("iteration 3:");
        let mut numbers = vec![3, 3, 3, 3, 3, 3, 3];
        Solution::remove_element(&mut numbers, 10);
        assert_eq!(numbers, vec![3, 3, 3, 3, 3, 3, 3]);

        println!("iteration 4:");
        let mut numbers = vec![1, 3, 3, 3, 3, 3, 3];
        Solution::remove_element(&mut numbers, 3);
        assert_eq!(numbers, vec![1, 3, 3, 3, 3, 3, 3]);

        println!("iteration 5:");
        let mut numbers = vec![1, 3, 3, 3, 3, 3, 1, 1];
        Solution::remove_element(&mut numbers, 3);
        assert_eq!(numbers, vec![1, 1, 1, 3, 3, 3, 1, 1]);

        println!("iteration 6:");
        let mut numbers = vec![2];
        Solution::remove_element(&mut numbers, 3);
        assert_eq!(numbers, vec![2]);
    }
}
