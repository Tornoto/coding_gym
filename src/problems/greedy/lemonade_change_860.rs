/// https://leetcode.com/problems/lemonade-change/
pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut five = 0;
    let mut ten = 0;
    for money in bills {
        if money == 5 {
            five += 1;
        } else if money == 10 {
            five -= 1;
            ten += 1;
            if five < 0 {
                return false;
            }
        } else {
            if ten > 0 {
                ten -= 1;
                five -= 1;
            } else {
                five -= 3;
            }

            if five < 0 {
                return false;
            }
        }
    }

    true
}
