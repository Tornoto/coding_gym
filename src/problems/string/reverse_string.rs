#![allow(unused)]

/// https://leetcode.cn/problems/reverse-string/description/
fn reverse_string<'a>(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
/// use unsafe version
fn reverse_string2(input: &mut String) {
    let bytes = unsafe { input.as_bytes_mut() };
    bytes[..].reverse();
}

/// https://leetcode.cn/problems/reverse-string-ii/description/
/// 使用 UnSafe
fn partial_reverse(s: &mut String, n: usize) {
    // Ensure `n` is within the bounds of the string length
    let len = s.len();
    if n > len {
        return;
    }

    // Convert the string to a vector of bytes
    let bytes: &mut [u8] = unsafe { s.as_bytes_mut() };

    // Reverse the first `n` bytes
    bytes[..n].reverse();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reverse_string() {
        let input = "abcdefg";
        assert_eq!(&reverse_string(&input), "gfedcba");
    }

    #[test]
    fn test_reverse_string2() {
        let mut input = "abcdefg".to_string();
        reverse_string2(&mut input);
        assert_eq!(&input, "gfedcba");
        println!(">> input: {}", input);
    }

    #[test]
    fn test_partial_reverse() {
        let mut original = String::from("Hello, world!");
        partial_reverse(&mut original, 5);
        println!("Original: {}", original); // Output: olleH, world!
    }
}
