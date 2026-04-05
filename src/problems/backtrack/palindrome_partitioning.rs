pub fn partition(s: String) -> Vec<Vec<String>> {
    if s.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut parts = vec![];
    partition_helper(&s, &mut result, &mut parts);
    result
}

fn partition_helper(s: &str, result: &mut Vec<Vec<String>>, parts: &mut Vec<String>) {
    if s.is_empty() {
        result.push(parts.clone());
        return;
    }

    let is_palindrome = |s: &str| -> bool {
        if s.is_empty() {
            return true;
        }
        let mut start = 0;
        let mut end = s.len();
        while start < end - 1 {
            if s[start..start + 1] != s[end - 1..end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    };

    // 注意切分点的边界为 1..=s.len()
    for cut_point in 1..=s.len() {
        let (first, second) = s.split_at(cut_point);
        if is_palindrome(first) {
            parts.push(first.to_owned());
            partition_helper(second, result, parts);
            parts.pop();
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_parition() {
        let s = "aab";
        let result = partition(s.to_string());
        println!(">> {:?}", result);
    }
}
