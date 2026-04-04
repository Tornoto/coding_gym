use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/reverse-words-in-a-string/
    pub fn reverse_words(s: String) -> String {
        // 辅助函数
        fn reverse_range(bytes: &mut [u8], start: usize, end: usize) {
            if start >= end {
                return;
            }

            let mut i = start;
            let mut j = end - 1; // j 指向有效区间的最后一个元素

            // 修正点：比较 i 和 j，而不是固定的 start
            while i < j {
                bytes.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        fn remove_extra_spaces(s: &mut String) {
            let bytes = unsafe { s.as_bytes_mut() };
            // slow pointer
            let mut write_idx = 0;
            // mark whether is space of new word
            // (used to reserve a single space between words)
            let mut prev_is_space = true;

            for i in 0..bytes.len() {
                let byte = bytes[i];

                if byte == b' ' {
                    if prev_is_space {
                        continue;
                    }
                    prev_is_space = true;
                } else {
                    prev_is_space = false;
                }

                bytes[write_idx] = byte;
                write_idx += 1;
            }

            // the loop above will keep a space after a word
            // we need to get rid of it
            if write_idx > 0 && bytes[write_idx - 1] == b' ' {
                write_idx -= 1;
            }

            s.truncate(write_idx);
        }

        let mut word = s;

        // 去除多余空格
        remove_extra_spaces(&mut word);

        // 整体反转
        let len = word.len();
        let mut bytes = unsafe { word.as_bytes_mut() };
        reverse_range(&mut bytes, 0, len);

        // 局部翻转
        let mut start = 0;
        for i in 0..=len {
            if i == len || bytes[i] == b' ' {
                reverse_range(&mut bytes, start, i);
                start = i + 1;
            }
        }

        word
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let words = String::from("the sky is blue");
        let res = Solution::reverse_words(words);
        println!("res: {:?}|", res);
        assert_eq!(res, "blue is sky the");
    }
}
