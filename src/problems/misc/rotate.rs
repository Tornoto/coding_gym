/**
 * leetcode 48. Rotate Image
 */
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut left = 0;
    let mut top = 0;
    let mut right = matrix.len() -1;
    let mut bottom = matrix.len() -1;
    while left < right {
        let mut step = 0;
        while step < right-left {
            rerange(matrix, left, top, right, bottom, step);
            step += 1;
        }
        left += 1;
        top += 1;
        right -= 1;
        bottom -= 1;
    }
}

pub fn rerange(matrix: &mut Vec<Vec<i32>>, left: usize, top: usize, right: usize, bottom: usize, step: usize) {
    let tmp = matrix[bottom-step][left];
    matrix[bottom-step][left] = matrix[bottom][right-step];
    matrix[bottom][right-step] = matrix[top+step][right];
    matrix[top+step][right] = matrix[top][left+step];
    matrix[top][left+step] = tmp;
}

#[test]
fn test_rotate() {
    let mut m1 = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
    let m1_r = vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]];
    rotate(&mut m1);

    let mut m2 = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    let m2_r = vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]];
    rotate(&mut m2);
    assert_eq!(m1, m1_r);
    assert_eq!(m2, m2_r);
}