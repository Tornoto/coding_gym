/// https://leetcode.com/problems/intersection-of-two-arrays/description/
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 0 <= nums1[i], nums2[i] <= 1000
    let mut set = vec![-1; 1001];
    for num in nums1 {
        if set[num as usize] == -1 {
            set[num as usize] = 0;
        }
    }

    let mut count = 0;
    for num in nums2 {
        if set[num as usize] == 0 {
            set[num as usize] = 1;
            count += 1;
        }
    }

    let mut result = Vec::with_capacity(count);
    for i in 0..set.len() {
        if set[i] == 1 {
            result.push(i as i32);
        }
    }

    result
}
