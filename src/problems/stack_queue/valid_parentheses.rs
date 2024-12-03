#[allow(unused)]
fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::with_capacity(s.len());

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                if let Some(top) = stack.last() {
                    match (top, ch) {
                        ('(', ')') | ('[', ']') | ('{', '}') => {
                            stack.pop();
                        }
                        _ => return false,
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let input = "(([]{})[]{})".to_string();
        let res = is_valid(input);
        println!("valid: {:?}", res);
    }
}
