/// https://leetcode.com/problems/split-a-string-in-balanced-strings/
pub fn balanced_string_split(s: String) -> i32 {
    let mut lcount = 0;
    let mut rcount = 0;

    let mut res = 0;

    for ch in s.chars() {
        if ch == 'L' {
            lcount += 1;
        }
        if ch == 'R' {
            rcount += 1;
        }

        if lcount == rcount {
            res += 1;
            lcount = 0;
            rcount = 0;
        }
    }

    res
}
