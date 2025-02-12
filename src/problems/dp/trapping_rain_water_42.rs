/// https://leetcode.com/problems/trapping-rain-water/description/
///
/// 接雨水这道题，想了很久，最终还是被AI教育了。
/// ```
/// □
/// □ O O □
/// □ O □ □
/// □ □ □ □
/// ```
/// 可以看到就像是沿着右侧的高度，用刷子刷齐一样（O表示雨水，□表示木桩）。想到这里，就差怎么计算出水的容量。
///
/// 其实，这个区域的水量，是每个柱子上水量的和。也就是需要知道这个柱子左右两侧的最大值。
///
/// 然后用较低的一侧，计算和当前柱子的高度差，求得当前柱子的存水量。
///
/// 这样问题就分解成：
/// 1. 找到每个柱子两侧的最大值；
/// 2. 计算每个柱子的水量
/// 3. 求和
///
/// 以下为动态规划求解方法
pub fn trap_dp(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut left_max = vec![0; n];
    let mut right_max = vec![0; n];

    // 从左向右遍历，获得柱子i左侧的柱子的最大高度
    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(height[i]);
    }

    // 从右向左遍历，获得柱子i右侧的柱子的最大高度
    right_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }

    // 计算收集的水量
    let mut volume = 0;
    for i in 0..n {
        volume += right_max[i].min(left_max[i]) - height[i];
    }

    volume
}

/// 如下为使用双指针解法
///
/// 在上面遍历数组获取 left_max 和 right_max 的过程中，
/// 我们似乎能感觉到，当需要获取左侧最大值的时候，我们从最左侧开始比较方便
/// 因为后续的节点能够用到其左侧的结果。
///
/// 而双指针则利用了这个特点，初始left_max(这里是一个usize，而非数组)和right_max指向数组两端
/// 总是更新矮的一侧。
///
/// 如果左侧矮，那么临近的元素能及时用到left_max。
pub fn trap_two_pointers(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut left = 0;
    let mut right = n - 1;
    let mut left_max = height[left];
    let mut right_max = height[right];

    let mut volume = 0;
    while left < right {
        if left_max < right_max {
            // 注意需要先移动指针，保证当前的柱子进行过计算
            // 不先移动指针可能导致 左指针到了一个柱子未计算，
            // 而有指针也到该柱子，且未计算 由于 right == left
            // 退出循环，导致该柱子的水量最终也没有计算
            // 如测试用例[2,0,5,1,2]
            left += 1;
            // 如果高度低于 left_max 说明可以存水
            if height[left] < left_max {
                volume += left_max - height[left];
            } else {
                left_max = height[left];
            }
        } else {
            right -= 1;
            if height[right] < right_max {
                volume += right_max - height[right];
            } else {
                right_max = height[right];
            }
        }
    }

    volume
}
