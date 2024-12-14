pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    if x % 10 == 0 {
        return false;
    }

    let mut org = x;
    let mut re = 0;
    
    while re < org {
        re = 10 * re + org % 10;
        org = org / 10;
        if re == org || org /10 == re {
            return true;
        }
    }
    return false;
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(0), true);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(120), false);
    assert_eq!(is_palindrome(9009), true);
    assert_eq!(is_palindrome(90509), true);
    assert_eq!(is_palindrome(90519), false);
}