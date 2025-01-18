pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    if strs.is_empty() || m == 0 || n == 0 {
        return 0;
    }

    let capacity_m = m as usize;
    let capacity_n = n as usize;
    let mut dp = vec![vec![0; capacity_n + 1]; capacity_m + 1];

    for str in strs.iter() {
        let one_count = str.chars().filter(|&c| c == '1').count();
        let zero_count = str.len() - one_count;
        for i in (zero_count..capacity_m + 1).rev() {
            for j in (one_count..capacity_n + 1).rev() {
                dp[i][j] = dp[i][j].max(dp[i - zero_count][j - one_count] + 1);
            }
        }
    }

    return dp[capacity_m][capacity_n] as i32;
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let strs = vec!["10", "0001", "111001", "1", "0"];
        let strs = strs
            .iter()
            .map(|&str| str.to_string())
            .collect::<Vec<String>>();
        let m = 5;
        let n = 3;
        let result = find_max_form(strs, m, n);
        println!("{:?}", result);

        let strs = vec!["10", "0", "1"];
        let strs = strs
            .iter()
            .map(|&str| str.to_string())
            .collect::<Vec<String>>();
        let m = 1;
        let n = 1;
        let result = find_max_form(strs, m, n);
        println!("{:?}", result);
    }
}
