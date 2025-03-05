#![allow(unused)]
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

// 注意两个版本在right的处理上。
//
// 在第一个版本中，right初始为 nums.len()，left = right 没有意义，因此循环的条件是 left < right
// 而且，更新right时，由于 nums[mid] > target，更新 right 为 mid 是安全的。这是因为如果更新right为mid-1，且nums[mid] == target
// 那么，由于 left 始终小于 right，而导致 mid 无法到达target的位置，从而查询不到。
//
// 在第二个版本中，right初始化为nums.len()-1，这种情况下 left = right 是有意义的，因此循环条件是 left <= right
// 而且，在更新right时，由于 nums[mid] > target，更新 right 为 mid-1。这因为left可能移动到right的位置。
fn binary_search_v2(nums: &[i32], target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid;
        }
        if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}

// 一种更符合rust风格的写法
fn binary_search_v3(nums: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
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
