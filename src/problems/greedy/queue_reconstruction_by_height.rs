/// https://leetcode.com/problems/queue-reconstruction-by-height/description/
pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // 根据身高降序、位置升序排序。
    // 这样遍历元素时，将元素插入对应位置是稳定的。
    // 思考：为什么不能按照身高升序排序呢？
    people.sort_by_key(|p| (-p[0], p[1]));
    let mut result = Vec::with_capacity(people.len());
    for person in people.iter() {
        // 根据当前 person 的位置，插入到结果对应下标处
        result.insert(person[1] as usize, person.to_vec());
    }
    result
}
