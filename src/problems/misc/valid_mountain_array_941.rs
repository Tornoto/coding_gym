/// https://leetcode.com/problems/valid-mountain-array/description/
pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return false;
    }

    let n = arr.len();
    let mut left = 0;
    let mut right = n - 1;

    while left < n - 1 && arr[left] < arr[left + 1] {
        left += 1;
    }

    while right > 0 && arr[right] < arr[right - 1] {
        right -= 1;
    }

    // 考虑特殊场景 [1,2,3] 和 [3,2,1]
    left == right && left != n - 1 && right != 0
}
