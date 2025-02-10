/// https://leetcode.cn/problems/daily-temperatures/description/
/// 整体思路，从左向右遍历气温数组，将气温对应的下标入栈
/// 如果即将入栈的温度高于栈内的温度，则小于该温度的弹出，并更新相差的天数；
/// 如果遍历完，栈内仍有元素，则其和”后面的最高温度“的天数间隔为默认的0。
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut stack = vec![0; n];
    let mut ans = vec![0; n];

    for i in 0..n {
        let temp = temperatures[i];
        // 将栈内小于 temp 的均弹出
        while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temp {
            let prev_idx = stack.pop().unwrap();
            ans[prev_idx] = (i - prev_idx) as i32;
        }
        // 将当前温度对应下标入栈
        stack.push(i);
    }

    ans
}
