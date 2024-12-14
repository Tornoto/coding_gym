#[allow(unused)]
fn binary_search(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    // 由于 while 循环的条件是 left < right
    // 为了能让长度为1的nums进入循环，
    // 设置 right 为数组长度
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2; // 防止溢出
        if nums[mid] == target {
            return mid as i32;
        }
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    -1
}

#[test]
fn test_binary_search() {
    let nums = vec![1, 3, 5, 7];
    assert_eq!(binary_search(&nums, 0), -1);
    assert_eq!(binary_search(&nums, 1), 0);
    assert_eq!(binary_search(&nums, 2), -1);
    assert_eq!(binary_search(&nums, 3), 1);
    assert_eq!(binary_search(&nums, 4), -1);
    assert_eq!(binary_search(&nums, 5), 2);
    assert_eq!(binary_search(&nums, 6), -1);
    assert_eq!(binary_search(&nums, 7), 3);
    assert_eq!(binary_search(&nums, 8), -1);
}
