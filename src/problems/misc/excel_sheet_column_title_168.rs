/// https://leetcode.com/problems/excel-sheet-column-title/
pub fn convert_to_title(column_number: i32) -> String {
    let mut column_number = column_number;
    let mut column_title = vec![];

    while column_number > 0 {
        // 注意减一
        column_number -= 1;
        let remainder: u8 = (column_number % 26) as u8;
        column_title.push((b'A' + remainder) as char);
        column_number /= 26;
    }

    column_title.iter().rev().collect()
}
