#[allow(unused)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_number() {
        let mut input: Vec<char> = "a1a11".chars().collect::<Vec<char>>();
        replace_number(&mut input, "num");
        println!("res: {}.", input.iter().collect::<String>());
    }
}
