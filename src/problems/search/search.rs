#![allow(unused)]
/// binary search
/// https://leetcode.com/problems/binary-search/description/
pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] > target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    None
}

#[test]
fn test() {
    let nums = vec![];
    assert_eq!(binary_search(&nums, 1), None);

    let nums = vec![1];
    assert_eq!(binary_search(&nums, 0), None);
    assert_eq!(binary_search(&nums, 1), Some(0));
    assert_eq!(binary_search(&nums, 2), None);

    let nums = vec![1, 2];
    assert_eq!(binary_search(&nums, 0), None);
    assert_eq!(binary_search(&nums, 1), Some(0));
    assert_eq!(binary_search(&nums, 2), Some(1));
    assert_eq!(binary_search(&nums, 3), None);

    let nums = vec![1, 2, 3];
    assert_eq!(binary_search(&nums, 0), None);
    assert_eq!(binary_search(&nums, 1), Some(0));
    assert_eq!(binary_search(&nums, 2), Some(1));
    assert_eq!(binary_search(&nums, 3), Some(2));
    assert_eq!(binary_search(&nums, 4), None);
}
