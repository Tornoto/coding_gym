pub fn sort_colors(nums: &mut Vec<i32>) {
    if nums.len() == 1 {
        return;
    }
    let mut left = 0;
    let mut right = nums.len();
    let mut i = 0;
    while i < right {
        if nums[i] == 1 {
            i += 1;
        } else if nums[i] == 0 {
            swap(nums, left, i);
            left += 1;
            i += 1;
        } else {
            swap(nums, right - 1, i);
            right -= 1;
        }
    }
}

fn swap(nums: &mut Vec<i32>, l: usize, r: usize) {
    let tmp = nums[l];
    nums[l] = nums[r];
    nums[r] = tmp;
}

#[test]
fn test_sort_colors() {
    let mut nums = vec![0, 2, 1, 0];
    sort_colors(&mut nums);
    assert_eq!(nums, vec!(0, 0, 1, 2));
}
