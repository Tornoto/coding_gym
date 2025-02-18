/// https://leetcode.com/problems/excel-sheet-column-number/
pub fn title_to_number(column_title: String) -> i32 {
    let mut acc = 1;
    let mut num = 0;
    for ch in column_title.chars().rev() {
        let val = ch as i32 - 'A' as i32 + 1;
        num += val * acc;
        acc *= 26;
    }

    return num;
}
