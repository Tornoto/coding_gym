pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    let mut result: Vec<i32> = digits
        .iter()
        .rev()
        .map(|val| {
            let sum = *val + carry;
            carry = sum / 10;
            sum % 10
        })
        .collect();
    if carry == 1 {
        result.push(1);
    }
    result.into_iter().rev().collect()
}

#[test]
fn test_plus_one() {
    assert_eq!(plus_one(vec!(1, 2, 3)), vec!(1, 2, 4));
    assert_eq!(plus_one(vec!(1, 2, 9)), vec!(1, 3, 0));
    assert_eq!(plus_one(vec!(9, 9, 9)), vec!(1, 0, 0, 0));
}
