#![allow(unused)]
use std::collections::VecDeque;

struct MyStack {
    vq1: VecDeque<i32>,
    vq2: VecDeque<i32>,
    first_in_use: bool,
}

impl MyStack {
    fn new() -> Self {
        Self {
            vq1: VecDeque::new(),
            vq2: VecDeque::new(),
            first_in_use: true,
        }
    }

    fn push(&mut self, x: i32) {
        if self.first_in_use {
            self.vq1.push_back(x);
        } else {
            self.vq2.push_back(x);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.first_in_use {
            while self.vq1.len() > 1 {
                self.vq2.push_back(self.vq1.pop_front().unwrap());
            }
            self.first_in_use = false;
            return self.vq1.pop_front();
        } else {
            while self.vq2.len() > 1 {
                self.vq1.push_back(self.vq2.pop_front().unwrap());
            }
            self.first_in_use = true;
            return self.vq2.pop_front();
        }
    }

    fn top(&mut self) -> Option<i32> {
        let result = self.pop();
        if result.is_some() {
            self.push(result.unwrap());
        }
        result
    }

    fn empty(&self) -> bool {
        self.vq1.is_empty() && self.vq2.is_empty()
    }
}
