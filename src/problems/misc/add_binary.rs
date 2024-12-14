/// https://leetcode.cn/problems/add-binary/description/
pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let (mut aiter, mut biter) = (a.chars().rev(), b.chars().rev());

    loop {
        let a_char = aiter.next();
        let b_char = biter.next();
        if a_char.is_none() && b_char.is_none() {
            break;
        }

        let a_char = a_char.unwrap_or('0');
        let b_char = b_char.unwrap_or('0');

        let a_bit = a_char.to_digit(2).unwrap();
        let b_bit = b_char.to_digit(2).unwrap();

        if (a_bit != 1 && a_bit != 0) || (b_bit != 1 && b_bit != 0) {
            panic!();
        }

        let sum = a_bit + b_bit + carry;
        carry = sum / 2;
        result.push(char::from_digit(sum % 2, 2).unwrap());
    }

    if carry > 0 {
        result.push('1');
    }
    result.chars().rev().collect::<String>()
}

#[test]
fn test_add_binary() {
    assert_eq!(
        add_binary("11".to_string(), "1".to_string()),
        "100".to_string()
    );
    assert_eq!(
        add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
    assert_eq!(
        add_binary("110010".to_string(), "10111".to_string()),
        "1001001".to_string()
    );
}
