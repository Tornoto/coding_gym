/// Given an m x n grid of blocks, where each block has an associated price, the task is to partition the entire grid into two rectangular subregions.
///
///  The partition must be made by a single straight cut, either horizontally or vertically.
///
/// The goal is to minimize the absolute difference between the total price of the blocks in the two resulting subregions.
pub fn developer_buy_land(blocks: Vec<Vec<i32>>) -> i32 {
    let m = blocks.len();
    let n = blocks[0].len();

    // calculate row sum and col sum
    let mut row_sum = vec![0; m];
    let mut col_sum = vec![0; n];
    for i in 0..m {
        row_sum[i] = blocks[i].iter().sum();
    }
    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            sum += blocks[j][i];
            col_sum[i] = sum;
        }
    }

    // calculate accumulate of row sum
    let mut row_acc = vec![0; m];
    let mut acc = 0;
    for i in 0..m {
        acc += row_sum[i];
        row_acc[i] = acc;
    }
    // calculate accumulate of col sum
    let mut col_acc = vec![0; n];
    let mut acc = 0;
    for i in 0..n {
        acc += col_sum[i];
        col_acc[i] = acc;
    }

    let mut min_diff = i32::MAX;

    for i in 0..m - 1 {
        let row_diff = (row_acc[m - 1] - 2 * row_acc[i]).abs();
        if row_diff < min_diff {
            min_diff = row_diff;
        }
    }

    for i in 0..n - 1 {
        let col_diff = (col_acc[n - 1] - 2 * col_acc[i]).abs();
        if col_diff < min_diff {
            min_diff = col_diff;
        }
    }

    min_diff
}

#[test]
fn test() {
    let blocks = vec![vec![1, 2, 3], vec![2, 1, 3], vec![1, 2, 3]];
    let diff = developer_buy_land(blocks);
    assert_eq!(diff, 0);

    let blocks = vec![vec![1, 2, 3], vec![2, 1, 3], vec![4, 5, 6]];
    let diff = developer_buy_land(blocks);
    assert_eq!(diff, 3);
}
