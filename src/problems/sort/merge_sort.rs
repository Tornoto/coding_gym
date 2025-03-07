pub fn merge_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let left = 0;
    let right = nums.len() - 1;
    merge_sort1(nums, left, right);
}

fn merge_sort1(nums: &mut [i32], left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;
        merge_sort1(nums, left, mid);
        merge_sort1(nums, mid + 1, right);
        merge(nums, left, mid, right);
    }
}

fn merge(nums: &mut [i32], left: usize, mid: usize, right: usize) {
    let n_left = mid - left + 1;
    let n_right = right - mid;
    let mut left_arr = vec![0; n_left];
    let mut right_arr = vec![0; n_right];

    for i in 0..n_left {
        left_arr[i] = nums[left + i];
    }

    for i in 0..n_right {
        right_arr[i] = nums[mid + i + 1];
    }

    let mut i = 0;
    let mut j = 0;
    let mut start = left;

    while i < n_left && j < n_right {
        if left_arr[i] <= right_arr[j] {
            nums[start] = left_arr[i];
            i += 1;
        } else {
            nums[start] = right_arr[j];
            j += 1;
        }
        start += 1;
    }

    while i < n_left {
        nums[start] = left_arr[i];
        start += 1;
        i += 1;
    }

    while j < n_right {
        nums[start] = right_arr[j];
        start += 1;
        j += 1;
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 5, 9, 1, 0, 3, 6, 8, 0, 0];
    merge_sort(&mut nums);
    println!("{:?}", nums);
}
