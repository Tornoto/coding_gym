/// https://leetcode.com/problems/spiral-matrix-ii/description/
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut row = 0;
    let mut col = 0;

    let mut result = vec![vec![0; n]; n];
    let mut width = n;

    let mut value = 1;
    while width > 1 {
        // always [a, b)
        // top line
        for i in col..col + width - 1 {
            result[row][i] = value;
            value += 1;
        }
        // right line
        for i in row..row + width - 1 {
            result[i][col + width - 1] = value;
            value += 1;
        }
        // bottom line
        for i in (col + 1..col + width).rev() {
            result[row + width - 1][i] = value;
            value += 1;
        }
        // left line
        for i in (row + 1..row + width).rev() {
            result[i][col] = value;
            value += 1;
        }
        row += 1;
        col += 1;
        width -= 2;
    }

    if n % 2 == 1 {
        result[n / 2][n / 2] = (n * n) as i32;
    }

    result
}

#[test]
fn test() {
    let result = generate_matrix(0);
    println!("{:?}", result);
    let result = generate_matrix(1);
    println!("{:?}", result);
    let result = generate_matrix(2);
    println!("{:?}", result);
    let result = generate_matrix(3);
    println!("{:?}", result);
    let result = generate_matrix(4);
    println!("{:?}", result);
    let result = generate_matrix(5);
    println!("{:?}", result);
}
