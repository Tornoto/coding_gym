# 双指针与滑动窗口模块

本模块收录了经典的双指针（Two Pointers）和滑动窗口（Sliding Window）算法题目。这些技巧是解决数组、字符串相关问题的核心方法，能够有效地将时间复杂度从 O(n²) 优化到 O(n)。

## 核心概念

### 双指针
双指针是指在遍历过程中使用两个指针，通过不同的移动策略来解决问题。常见模式包括：

| 模式 | 描述 | 典型场景 |
|------|------|----------|
| 相向指针 | 两指针分别从两端向中间移动 | 有序数组查找、反转操作 |
| 同向指针 | 两指针同向移动，维护一个窗口 | 子数组/子串问题 |
| 快慢指针 | 一快一慢，用于删除/原地修改 | 数组去重、移动元素 |

### 滑动窗口
滑动窗口是双指针的重要应用，通过维护一个动态的窗口来解决连续子数组/子串问题：

```
窗口扩张：右指针右移，加入新元素
窗口收缩：左指针右移，移除旧元素
```

## 题目列表

### 🟢 入门题目

| 题目 | 文件 | 核心技巧 | 难度 |
|------|------|----------|------|
| 两数之和 | `two_sum.rs` | 哈希表 + 双指针变种 | 简单 |
| 移除元素 | `remove_element.rs` | 快慢指针 | 简单 |
| 删除重复元素 | `remove_duplicates.rs` | 快慢指针 | 简单 |
| 移动零 | `move_zeroes.rs` | 快慢指针 | 简单 |
| 替换数字 | `replace_number.rs` | 双指针原地修改 | 简单 |

### 🟡 进阶题目

| 题目 | 文件 | 核心技巧 | 难度 |
|------|------|----------|------|
| 有序数组的平方 | `sorted_squares.rs` | 相向双指针 | 简单 |
| 反转字符串中的单词 | `reverse_words.rs` | 双指针 + 原地反转 | 中等 |
| 比较含退格的字符串 | `backspace_compare.rs` | 相向双指针 | 简单 |
| 三数之和 | `three_sum.rs` | 排序 + 双指针 | 中等 |
| 四数之和 | `four_sum.rs` | 排序 + 双指针嵌套 | 中等 |

### 🔴 滑动窗口专题

| 题目 | 文件 | 核心技巧 | 难度 |
|------|------|----------|------|
| 水果入篮 | `fruit_into_baskets_904.rs` | 滑动窗口 + HashMap | 中等 |
| 长度最小的子数组 | `minimum_size_subarray_sum_209.rs` | 滑动窗口 + 前缀和 | 中等 |
| 最小覆盖子串 | `minimum_window_substring_76.rs` | 滑动窗口 + 计数 | 困难 |

## 模板代码

### 滑动窗口通用模板

```rust
fn sliding_window_template(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut result = 0;
    
    for right in 0..nums.len() {
        // 1. 将右边界元素加入窗口
        // add_element(nums[right]);
        
        // 2. 当窗口不满足条件时，收缩左边界
        // while !window_is_valid() {
        //     remove_element(nums[left]);
        //     left += 1;
        // }
        
        // 3. 更新结果
        // result = result.max(window_size);
    }
    
    result
}
```

### 快慢指针通用模板

```rust
fn fast_slow_pointer_template(nums: &mut Vec<i32>) -> i32 {
    let mut slow = 0;
    
    for fast in 0..nums.len() {
        // 当满足某个条件时，移动慢指针
        if should_move_slow(nums[fast]) {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    
    slow as i32  // 返回新长度
}
```

## 使用指南

1. **运行单个题目测试**
   ```bash
   cargo test --package coding_gym --lib problems::two_pointers::<module>::test
   ```

2. **运行本模块所有测试**
   ```bash
   cargo test --package coding_gym --lib problems::two_pointers
   ```

## 学习建议

1. 先从**入门题目**开始，理解快慢指针的基本操作
2. 掌握**相向指针**在有序数组中的应用
3. 重点练习**滑动窗口**，理解窗口扩张和收缩的时机
4. 挑战**困难题目**，综合运用多种技巧

## 参考资源

- [LeetCode 双指针标签](https://leetcode.com/tag/two-pointers/)
- [LeetCode 滑动窗口标签](https://leetcode.com/tag/sliding-window/)