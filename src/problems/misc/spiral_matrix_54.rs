/// https://leetcode.com/problems/spiral-matrix/description/
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut result = Vec::with_capacity(m * n);
    let mut sr = 0;
    let mut sc = 0;
    let mut width = n;
    let mut height = m;
    while width > 0 && height > 0 {
        // when width or height is 1, just go through the vertical/horizontal line
        if width == 1 {
            for i in sr..sr + height {
                result.push(matrix[i][sc]);
            }
            break;
        }

        if height == 1 {
            for i in sc..sc + width {
                result.push(matrix[sr][i]);
            }
            break;
        }

        // range always is [a,b)
        // top line
        for i in sc..sc + width - 1 {
            result.push(matrix[sr][i]);
        }
        // right line
        for i in sr..sr + height - 1 {
            result.push(matrix[i][sc + width - 1]);
        }
        // bottom line
        for i in (sc + 1..sc + width).rev() {
            result.push(matrix[sr + height - 1][i]);
        }
        // left line
        for i in (sr + 1..sr + height).rev() {
            result.push(matrix[i][sc]);
        }

        sr += 1;
        sc += 1;
        height -= 2;
        width -= 2;
    }

    result
}

#[test]
fn test() {
    let matrix = vec![vec![1]];
    let result = spiral_order(matrix);
    println!("{:?}", result);

    let matrix = vec![vec![1, 2]];
    let result = spiral_order(matrix);
    println!("{:?}", result);
    let matrix = vec![vec![1, 2, 3]];
    let result = spiral_order(matrix);
    println!("{:?}", result);

    let matrix = vec![vec![1], vec![2]];
    let result = spiral_order(matrix);
    println!("{:?}", result);

    let matrix = vec![vec![1], vec![2], vec![3]];
    let result = spiral_order(matrix);
    println!("{:?}", result);

    let matrix = vec![vec![1, 2], vec![3, 4]];
    let result = spiral_order(matrix);
    println!("{:?}", result);

    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let result = spiral_order(matrix);
    println!("{:?}", result);

    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = spiral_order(matrix);
    println!("{:?}", result);
}
