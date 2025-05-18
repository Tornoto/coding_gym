/// given an array of 1*n, and several ranges, output the sums of these ranges
///
/// for example:
/// - array: 1,2,3,4,5
/// - ranges1: (0, 1), (0,4), (1,2)...
pub fn sums_of_subarrays(nums: Vec<i32>, ranges: Vec<(usize, usize)>) -> Vec<i32> {
    // 利用前缀和避免反复累加区间数值。
    // accumulate[i] = sum[0..=i];
    // accumulate[j] = sum[0..=j];
    // sum[i..=j] = accumulate[j] - accumulate [i] + num[i];
    // = accumulate[j] - ( accumulate [i] - num[i]);
    // = accumulate[j] - accumulate[i-1] (if i > 0)
    let mut acc = vec![0; nums.len()];
    let mut sum = 0;
    for i in 0..nums.len() {
        sum += nums[i];
        acc[i] = sum;
    }

    let mut result = Vec::<i32>::with_capacity(ranges.len());

    for (start, end) in ranges {
        if start > 0 {
            result.push(acc[end] - acc[start - 1]);
        } else {
            result.push(acc[end]);
        }
    }

    result
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5];
    let ranges = vec![(0, 1), (1, 3)];
    let res = sums_of_subarrays(nums, ranges);
    assert_eq!(res, vec![3, 9]);
}
