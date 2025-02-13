/// https://leetcode.com/problems/largest-rectangle-in-histogram/description/
///
/// 这里讨论下单调栈中元素的特点，空心框组成□的代表弹出的柱子，■表示在栈中的柱子。
/// ```
///  □
///  □□ ■
///  □□■■
/// ■□□■■
/// ■□□■■
/// 01234
/// ```
/// 我们来看0号柱子和3号柱子，虽然实际栈中这两者紧挨着，但曾经两者中间的元素弹出过栈。
///
/// 也就是说，栈中两个元素挨着，不代表坐标是连续的。
///
/// 那么紧挨着的两个柱子，其间的柱子有什么特点？一定是高于左右柱子的。
///
/// 还是以0号3号柱子为例，
/// - 如果 1 2 柱子高度小于0号柱子，那么0号会被弹出，因此 1 2 柱子定不小于0号柱子；
/// - 如果高度等于0号柱子，那么 1 2 还会保留在栈中，因此 1 2 必然大于 0 号柱子
/// - 如果高度小于3等于号柱子，那么 1 2 也会保留在栈中，因此1 2 必然不小于3号柱子；
/// - 因此，可以得出结论 1 2 柱子必然高于3号和0号柱子。
///
/// 有了这样的结论，当弹出3号柱子计算面积时，此时i时4，那么(0,3]区间的柱子高度定然是大于等于3号柱子的，因此宽度是 4-0-1
///
/// 再看另外一个场景，即弹出某个元素后，栈内为空
/// ```
/// ■
/// ■ □
/// ■ □
/// x i
/// ```
/// 假设此时i要入栈，x号柱子要弹出，高度大于等于heights[2]柱子（数量）宽度是多少呢？
///
/// 根据上面的推论，我们知道x之前的元素定然是要高于x柱的，否则高度小于x的柱子是会留在栈中的。
///
/// 那么x 和i 之间的柱子高度呢？同样，也必然是要高于x的，如果x 和i之间的柱子低于x柱，那么x柱会在之前的循环中弹出。
///
/// 因此我们可以恢复一种可能的历史栈
/// ```
///  □  □
/// □□ □□
/// □□■□□
/// □□■□□    □
/// □□■□□    □
/// 01234    5
/// ```
/// 因此宽度就是i(5)，height 是 heights[2]
///
/// 这样就可以知道为什么如此选取height和width了
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::with_capacity(n);

    let mut max_area = 0;
    for i in 0..=n {
        let cur_height = if i == n { 0 } else { heights[i] };
        while !stack.is_empty() && cur_height < heights[*stack.last().unwrap()] {
            let height = heights[stack.pop().unwrap()];
            let width = if stack.is_empty() {
                i
            } else {
                i - stack.last().unwrap() - 1
            };
            max_area = max_area.max(height * width as i32);
        }

        stack.push(i);
    }

    max_area
}
