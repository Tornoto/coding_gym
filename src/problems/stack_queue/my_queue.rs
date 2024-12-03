#![allow(unused)]
/// https://leetcode.com/problems/implement-queue-using-stacks/description/
struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            stack_in: vec![],
            stack_out: vec![],
        }
    }

    /// as we have two stacks, new element can be pushed into stack_in
    /// however, after popping out an element,
    /// can we still push new element into stack_in directly?
    /// the answer is yes.
    /// example: stack_in: [] stack_out:[4, 3, 2] (after popping out 1)
    /// push op: stack_in: [5] stack_out:[4, 3, 2]
    /// the general order keeps unchanged: 5, 4, 3, 2.
    /// the next element to pop is 2 and the last is 5.
    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    /// suppose elements are in stack_in,
    /// we need to pop the elements and push into stack_out
    /// then we can pop the stack_out one time to get the result
    /// if stack_out is not empty, we can pop stack_out directly
    fn pop(&mut self) -> Option<i32> {
        if self.stack_out.is_empty() {
            while let Some(item) = self.stack_in.pop() {
                self.stack_out.push(item);
            }
        }
        self.stack_out.pop()
    }

    fn peek(&mut self) -> Option<i32> {
        if self.stack_out.is_empty() {
            while let Some(item) = self.stack_in.pop() {
                self.stack_out.push(item);
            }
        }
        self.stack_out.last().cloned()
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}
