/// https://leetcode.com/problems/rotate-array/
///
/// 按某个轴旋转，可以转化为
/// - 整体翻转
/// - 前半部分翻转
/// - 后部分翻转
/// ```
/// 初始状态
/// [1,2,3,4,5,6,7]
///          ^
/// 整体翻转
/// [7,6,5,4,3,2,1]
///      ^
/// 后半部分翻转
/// [7,6,5,1,2,3,4]
///      ^
/// 前半部分翻转
/// [5,6,7,1,2,3,4]
/// ```
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k = k as usize % n;
    let nums = nums;
    nums[0..].reverse();
    nums[0..k].reverse();
    nums[k..].reverse();
}
