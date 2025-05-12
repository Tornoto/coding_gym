#![allow(unused)]
/// https://leetcode.com/problems/backspace-string-compare/description/
pub fn backspace_compare(s: String, t: String) -> bool {
    stack_solution(&s, &t)
    // two_pointer_solution(&s, &t)
}

/// Time Complexity: O(M+N)
/// Space Complexity: O(M+N)
fn stack_solution(s: &str, t: &str) -> bool {
    build_string(&s) == build_string(&t)
}

fn build_string(s: &str) -> String {
    let mut stack = Vec::with_capacity(s.len());

    for ch in s.chars() {
        if ch == '#' {
            if !stack.is_empty() {
                stack.pop();
            }
        } else {
            stack.push(ch);
        }
    }

    stack.iter().collect::<String>()
}

/// iterate reversely
fn two_pointer_solution(s: &str, t: &str) -> bool {
    // todo
    true
}

#[test]
fn test() {
    let s = "xywrrmp".to_string();
    let t = "xywrrmu".to_string();
    assert_eq!(backspace_compare(s, t), false);

    let s = "x#y#".to_string();
    let t = "xy##".to_string();
    assert_eq!(backspace_compare(s, t), true);

    let s = "x##y".to_string();
    let t = "#x#y".to_string();
    assert_eq!(backspace_compare(s, t), true);

    let s = "xywrrmp".to_string();
    let t = "xywrrmu#p".to_string();
    assert_eq!(backspace_compare(s, t), true);
}
