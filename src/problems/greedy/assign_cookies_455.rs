/// https://leetcode.com/problems/assign-cookies/description/
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    let mut s = s;
    g.sort_unstable();
    s.sort_unstable();

    let (mut child, mut cookie) = (0, 0);

    while child < g.len() && cookie < s.len() {
        // 尝试用当前的饼干满足当前的孩子
        if s[cookie] >= g[child] {
            // 孩子被满足了，移动到下一个孩子
            child += 1;
        }
        // 不论是否满足当前孩子，都要移动到下一个饼干
        cookie += 1;
    }

    child as i32
}
