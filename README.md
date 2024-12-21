# Coding Gym

使用 Rust 语言实现的编程练习项目，包含多种算法和数据结构题目。

## 问题分类

### 栈和队列

- [计算逆波兰表达式](src/problems/stack_queue/eval_rpn.rs)

### 字符串处理

- [二进制数相加](src/problems/misc/add_binary.rs)
- [判断回文数](src/problems/misc/is_palindrome.rs)
- [字母异位词分组](src/problems/misc/group_anagrams.rs)

### 树相关问题

- [判断相同的树](src/problems/misc/is_same_tree.rs)

### 链表操作

- [两数相加(链表实现)](src/problems/misc/add_two_numbers.rs)

### 数组和查找

- [二分查找](src/problems/misc/binary_search.rs)
- [寻找两个正序数组的中位数](src/problems/misc/find_median_sorted_arrays.rs)

## 项目结构

```bash
src/
├── problems/           # 问题解决方案
│   ├── misc/          # 混合问题
│   ├── stack_queue/   # 栈和队列相关问题
│   ├── string/        # 字符串处理问题
│   ├── tree/          # 树相关问题
│   └── two_pointers/  # 双指针技巧相关问题
```
