/// https://leetcode.com/problems/largest-number
pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>();

    nums.sort_by(|str1, str2| compare(str1, str2));

    // 如果是"000"应该返回"0"
    // 因为已经排序，如果开头是0，则说明后续全是0
    if nums[0] == "0" {
        return "0".to_string();
    }

    nums.join("")
}

// 逆序
fn compare(str1: &str, str2: &str) -> std::cmp::Ordering {
    if str1.len() == str2.len() {
        return str2.cmp(str1);
    } else {
        let min_len = str1.len().min(str2.len());
        if str1[0..min_len] == str2[0..min_len] {
            if str1.len() == min_len {
                return compare(str1, &str2[min_len..]);
            } else {
                return compare(&str1[min_len..], str2);
            }
        } else {
            return str2.cmp(str1);
        }
    }
}
