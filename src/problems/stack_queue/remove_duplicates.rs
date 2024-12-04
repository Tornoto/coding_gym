pub fn remove_duplicates(s: String) -> String {
    let mut stack = Vec::<char>::with_capacity(s.len());

    for ch in s.chars() {
        if let Some(top) = stack.pop() {
            if top != ch {
                stack.push(top);
                stack.push(ch);
            }
        } else {
            stack.push(ch);
        }
    }

    stack.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let input: String = "abccbabc".to_string();
        let res = remove_duplicates(input);
        println!("res: {:?}", res);

        let input: String = "abcdef".to_string();
        let res = remove_duplicates(input);
        println!("res: {:?}", res);
    }
}
