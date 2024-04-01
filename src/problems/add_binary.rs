pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = '0';
    let (mut aiter, mut biter) = (a.chars().rev(), b.chars().rev());
    loop {
        match (aiter.next(), biter.next()) {
            (Some('1'), Some('1')) => {
                result.push(carry);
                carry = '1';
            }
            (Some('1'), Some('0')) | (Some('0'), Some('1')) | (None, Some('1')) | (Some('1'), None)  => {
                if carry == '1' {
                    result.push('0');
                } else {
                    result.push('1');
                    carry = '0';
                }
            }
            (Some('0'), Some('0')) | (None, Some('0')) | (Some('0'), None) => {
                result.push(carry);
                carry = '0';
            }
            _ => break,
        }
    }
    if carry == '1' {
        result.push(carry);
    }
    result.chars().rev().collect::<String>()
}

#[test]
fn test_add_binary(){
    assert_eq!(add_binary("11".to_string(), "1".to_string()),"100".to_string());
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()),"10101".to_string());
    assert_eq!(add_binary("110010".to_string(), "10111".to_string()),"1001001".to_string());
}