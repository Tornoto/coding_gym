pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::<String>::with_capacity(tokens.len());

    for token in tokens {
        match token.as_str() {
            "+" => {
                let (right, left) = get_operand(&mut stack);
                let cur_res = left + right;
                stack.push(cur_res.to_string());
            }
            "-" => {
                let (right, left) = get_operand(&mut stack);
                let cur_res = left - right;
                stack.push(cur_res.to_string());
            }
            "*" => {
                let (right, left) = get_operand(&mut stack);
                let cur_res = left * right;
                stack.push(cur_res.to_string());
            }
            "/" => {
                let (right, left) = get_operand(&mut stack);
                let cur_res = left / right;
                stack.push(cur_res.to_string());
            }
            _ => {
                stack.push(token.clone());
            }
        }
    }

    stack.pop().unwrap().parse().unwrap()
}

fn get_operand(stack: &mut Vec<String>) -> (i32, i32) {
    (
        stack.pop().unwrap().parse().unwrap(),
        stack.pop().unwrap().parse().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        let tokens: Vec<String> = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        let res = eval_rpn(tokens);
        println!("res: {:?}", res);
    }
}
