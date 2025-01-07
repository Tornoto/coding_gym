/// https://leetcode.cn/problems/minimum-number-of-arrows-to-burst-balloons/description/
pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut points = points;
    // 按结束从先到后排序
    points.sort_unstable_by_key(|val| (val[1]));

    let mut count = 1;
    let mut cur_point = &points[0];
    for idx in 1..n {
        // 如果起球起始点在当前起球结束点之后，则需要新的一箭
        if points[idx][0] > cur_point[1] {
            count += 1;
            cur_point = &points[idx];
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let count = find_min_arrow_shots(points);
        assert_eq!(2, count);

        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let count = find_min_arrow_shots(points);
        assert_eq!(4, count);

        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let count = find_min_arrow_shots(points);
        assert_eq!(2, count);

        let points = vec![vec![1, 2]];
        let count = find_min_arrow_shots(points);
        assert_eq!(1, count);

        let points = vec![vec![1, 2], vec![1, 2]];
        let count = find_min_arrow_shots(points);
        assert_eq!(1, count);
    }
}
