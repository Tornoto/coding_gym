use std::{collections::HashSet, hash::Hash};

/// https://leetcode.com/problems/longest-consecutive-sequence/description/
/// 说一下这个超时解法的思路。比较丑陋。
/// 用一个set判断数字有没有访问过
/// 用一个set数组存放一些连续的序列
/// 对于一个数字，如果没有访问过，再判断num-1和num+1有没有访问过
/// 如果均访问过，则说明存在两个set分别包含了num-1和num+1，需要合并这两个集合，然后在将num放入集合中；
/// 如果访问过其中一个，则需要将num放到对应的集合中
/// 这种解法耗费空间，而且需要遍历set数组才能找到存放num-1或num+1的集合。
/// 如果输入数组中的数组均不连续，则退化到n^2
pub fn longest_consecutive_timeout(nums: Vec<i32>) -> i32 {
    let mut seqs = vec![];
    let mut visited: HashSet<i32> = HashSet::new();

    for num in nums {
        if !visited.contains(&num) {
            if !visited.contains(&(num - 1)) && !visited.contains(&(num + 1)) {
                let mut sub_set = HashSet::new();
                sub_set.insert(num);
                seqs.push(sub_set);
            } else if !visited.contains(&(num - 1)) {
                for seq in &mut seqs {
                    if seq.contains(&(num + 1)) {
                        seq.insert(num);
                    }
                }
            } else if !visited.contains(&(num + 1)) {
                for seq in &mut seqs {
                    if seq.contains(&(num - 1)) {
                        seq.insert(num);
                    }
                }
            } else {
                let mut small = None;
                let mut large = None;
                for i in 0..seqs.len() {
                    if seqs[i].contains(&(num - 1)) {
                        small = Some(i);
                    }
                    if seqs[i].contains(&(&num + 1)) {
                        large = Some(i);
                    }
                    if small.is_some() && large.is_some() {
                        if small.unwrap() > large.unwrap() {
                            let tmp = small.clone();
                            small = large;
                            large = tmp;
                        }
                        break;
                    }
                }
                if small.is_some() && large.is_some() {
                    let large_set = seqs.swap_remove(large.unwrap());
                    seqs[small.unwrap()].extend(large_set);
                    seqs[small.unwrap()].insert(num);
                }
            }
            visited.insert(num);
        }
    }

    let mut max_len = 0;
    for i in 0..seqs.len() {
        max_len = max_len.max(seqs[i].len());
    }

    return max_len as i32;
}

/// 高效的解法
/// 考虑连续数组的特点，如果原始输入中存在连续序列，假设序列的起始为a，那么a-1必然不在数组中。
/// 因此可以先考虑将数组元素放入set中。
/// 遍历集合，如果a-1在set中，则说明a不是序列的起始，则跳过该元素
/// 如果a-1不在set中，则说明a是序列的起始，则进入内部循环，从a+1开始递增，如果nums[i]+1在set中，则更新序列长度
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // 记录数组中的数值
    let num_set: HashSet<i32> = nums.into_iter().collect();

    let mut longest_streak = 0;
    for num in num_set.iter() {
        let mut num = *num;
        // 如果集合包含 nums[i]-1 则说明 nums[i] 不是序列起始
        if num_set.contains(&(num - 1)) {
            continue;
        }

        let mut cur_streak = 1;
        // 递增num，判断子序列长度
        while num_set.contains(&(num + 1)) {
            num += 1;
            cur_streak += 1;
        }
        longest_streak = longest_streak.max(cur_streak);
    }

    longest_streak
}

#[test]
fn test() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let length = longest_consecutive_timeout(nums);
    println!("{:?}", length);
}
