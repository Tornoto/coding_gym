#![allow(unused)]
// https://leetcode.com/problems/remove-element/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut start: usize = 0;
    let mut end: usize = nums.len();

    // it's safe and clean to use start < end rather than start + 1 < end
    // for using start + 1 < end will not enter the loop while the array has only one ele.
    while start < end {
        // move end to the first element that does not equal target
        while end > 0 && nums[end - 1] == val {
            end -= 1;
        }

        if end < 1 {
            return start as i32;
        }
        // move start to the first element that equals target
        while start < nums.len() && nums[start] != val {
            start += 1;
        }

        if start + 1 < end {
            nums.swap(start, end - 1);
        }
    }

    start as i32
}

/// https://kamacoder.com/problempage.php?pid=1064
/// 默认替换数字为 "num"
fn replace_number(input: &mut Vec<char>, ptn: &str) {
    if ptn.is_empty() {
        return;
    }
    let ptn_vec = ptn.chars().collect::<Vec<char>>();
    // count the number of num
    let mut num_count = 0;
    for ch in input.iter() {
        if ch.is_numeric() {
            num_count += 1;
        }
    }
    let old_len = input.len();
    let new_len = input.len() + num_count * (ptn.len() - 1);
    // resize the vec and fill with space
    input.resize(new_len, ' ');

    let mut p1 = old_len;
    let mut p2 = new_len;

    while p1 > 0 {
        let ch = input[p1 - 1];
        if ch.is_numeric() {
            input.splice(p2 - ptn.len()..p2, ptn_vec.clone());
            p2 -= ptn.len();
        } else {
            input[p2 - 1] = ch;
            p2 -= 1;
        }
        p1 -= 1;
    }
    println!("size of input: {:?}", input.len());
}

/// https://leetcode.com/problems/reverse-words-in-a-string/
/// pass leetcode
pub fn reverse_words(s: String) -> String {
    println!("full len: {:?}", s.len());
    let mut words: Vec<char> = s.chars().rev().collect();

    let mut start: usize = 0;
    let end = words.len();

    while start < end {
        let (space_count, fsans) = find_first_space_after_none_space(&words, start);
        words[start..fsans].reverse();
        println!("itr: {:?}|", words.clone().into_iter().collect::<String>());
        start = fsans - space_count + 1;
        println!("start, fsans: {:?}", (start, fsans));
        if fsans == end && words[fsans - 1] == ' ' {
            break;
        }
    }

    for last_none_space in (0..end).rev() {
        if words[last_none_space] != ' ' {
            words.truncate(last_none_space + 1);
            break;
        }
    }
    words.into_iter().collect()
}

fn find_first_space_after_none_space(s: &[char], start: usize) -> (usize, usize) {
    // skip starting spaces
    let mut idx = start;
    let mut space_count: usize = 0;
    while idx < s.len() && s[idx] == ' ' {
        space_count += 1;
        idx += 1;
    }

    while idx < s.len() && s[idx] != ' ' {
        idx += 1;
    }
    (space_count, idx)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        println!("iteration 1:");
        let mut numbers = vec![1, 2, 3, 2, 3, 3, 4];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 2:");
        let mut numbers = vec![3, 3, 3, 3, 3, 3, 3];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 3:");
        let mut numbers = vec![3, 3, 3, 3, 3, 3, 3];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 10);
        println!("{:?}", numbers);

        println!("iteration 4:");
        let mut numbers = vec![1, 3, 3, 3, 3, 3, 3];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 5:");
        let mut numbers = vec![1, 3, 3, 3, 3, 3, 1, 1];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);

        println!("iteration 6:");
        let mut numbers = vec![2];
        println!("{:?}", numbers);
        remove_element(&mut numbers, 3);
        println!("{:?}", numbers);
    }

    #[test]
    fn test_replace_number() {
        let mut input: Vec<char> = "a1a11".chars().collect::<Vec<char>>();
        replace_number(&mut input, "num");
        println!("res: {}.", input.iter().collect::<String>());
    }

    #[test]
    fn test_reverse_words() {
        let words = String::from(" hello world!  from rust abc  abc ");
        let res = reverse_words(words);
        println!("res: {:?}|", res);
    }
}
