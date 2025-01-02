/// https://leetcode.com/problems/jump-game-ii/description/
pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    // 初始化 coverage 为，如果nums[0] = 1，即意味着能覆盖到下标为 1 的位置
    let mut coverage = nums[0];
    // 提前退出条件
    if coverage + 1 >= nums.len() as i32 {
        return 1;
    }

    // 初始化当前 coverage 覆盖下的某个节点能扩展到的最大覆盖范围
    let mut region = 0;
    // 默认跳数初始化为 1
    let mut jumps = 1;
    for (index, value) in nums.iter().enumerate() {
        // 计算当前节点的覆盖范围
        let tmp_region = index as i32 + value;
        // 如果大于当前 region，则更新
        if tmp_region > region {
            region = tmp_region;
        }
        // 如果达到当前覆盖范围的边界，或者达到数组末尾
        // 则增加一跳；并将 region 置为 0；条件更新 coverage。
        // 注意，并不关心具体跳到哪里，在边界范围内必跳，只是在边界处才更新跳数
        // 并考虑到数组结尾的情形
        if index == coverage as usize || index == nums.len() - 1 {
            if region > coverage {
                coverage = region;
            }
            jumps += 1;
            region = 0;
        }

        if coverage >= nums.len() as i32 - 1 {
            return jumps;
        }
    }

    jumps
}
