#[allow(unused)]
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
    fn test_reverse_words() {
        let words = String::from(" hello world!  from rust abc  abc ");
        let res = reverse_words(words);
        println!("res: {:?}|", res);
    }
}
