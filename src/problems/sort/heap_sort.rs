// 从大到小排序
pub fn heap_sort(nums: &mut [i32]) {
    let n = nums.len();
    // 初始化大堆
    for i in (0..n / 2).rev() {
        heapify(nums, n, i);
    }

    println!("init: {:?}", nums);

    for i in (0..n).rev() {
        nums.swap(0, i);
        heapify(nums, i, 0);
    }
}

// 大堆
fn heapify(nums: &mut [i32], n: usize, i: usize) {
    let largest = get_largest(nums, n, i);

    if largest != i {
        nums.swap(largest, i);
        heapify(nums, n, largest);
    }
}

fn get_largest(nums: &[i32], n: usize, i: usize) -> usize {
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    let mut largest = i;
    if left < n && nums[left] > nums[largest] {
        largest = left;
    }

    if right < n && nums[right] > nums[largest] {
        largest = right;
    }

    largest
}

// 从小到大排序
pub fn heap_sort_rev(nums: &mut [i32]) {
    let n = nums.len();
    // 初始化小堆
    for i in (0..n / 2).rev() {
        heapify_rev(nums, n, i);
    }

    for i in (0..n).rev() {
        nums.swap(i, 0);
        heapify_rev(nums, n, 0);
    }
}

fn heapify_rev(nums: &mut [i32], n: usize, i: usize) {
    let smallest = get_smallest(nums, n, i);

    if smallest != i {
        nums.swap(smallest, i);
        heapify_rev(nums, n, smallest);
    }
}

fn get_smallest(nums: &mut [i32], n: usize, i: usize) -> usize {
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    let mut smallest = i;
    if left < n && nums[left] < nums[smallest] {
        smallest = left;
    }
    if right < n && nums[right] < nums[smallest] {
        smallest = right;
    }

    smallest
}
