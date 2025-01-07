/// https://leetcode.com/problems/non-overlapping-intervals/description/
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let n = intervals.len();
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|i| i[1]);

    let mut count = 0;
    let mut cur_iterval = &intervals[0];
    for idx in 1..n {
        if intervals[idx][0] < cur_iterval[1] {
            count += 1;
        } else {
            cur_iterval = &intervals[idx];
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        let count = erase_overlap_intervals(intervals);
        assert_eq!(count, 1);

        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
        let count = erase_overlap_intervals(intervals);
        assert_eq!(count, 2);

        let intervals = vec![vec![1, 2], vec![2, 3]];
        let count = erase_overlap_intervals(intervals);
        assert_eq!(count, 0);
    }
}
