/// https://leetcode.com/problems/merge-intervals/description/
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    const START: usize = 0;
    const END: usize = 1;

    let mut intervals = intervals;
    // 按区间起始升序
    intervals.sort_unstable_by_key(|k| k[START]);

    let mut merged_intervals = vec![];
    let mut start = intervals[0][START];
    let mut end = intervals[0][END];
    for interval in intervals {
        // 如果当前区间的起始小于 end
        // 则更新 end 为 重叠区间中较大的
        if interval[START] <= end {
            end = end.max(interval[END]);
        } else {
            // 如果起始在 end 之后，遇到不重叠区间
            // 则添加到结果集中，并更新 start 和 end
            merged_intervals.push(vec![start, end]);
            start = interval[START];
            end = interval[END];
        }
    }
    merged_intervals.push(vec![start, end]);

    merged_intervals
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let res = merge(intervals);
        assert_eq!(res, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);

        let intervals = vec![vec![1, 4], vec![4, 5]];
        let res = merge(intervals);
        assert_eq!(res, vec![vec![1, 5]]);

        let intervals = vec![vec![2, 3], vec![1, 4]];
        let res = merge(intervals);
        assert_eq!(res, vec![vec![1, 4]]);
    }
}
