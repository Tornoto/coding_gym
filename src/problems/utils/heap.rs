#![allow(unused)]

use std::fmt::Debug;

#[derive(Debug)]
pub struct BinHeap<T: Ord + Debug> {
    data: Vec<T>,
}

impl<T: Ord + Debug> BinHeap<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::<T>::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let size = self.data.len();
        self.data.swap(0, size - 1);
        let max = self.data.pop();
        self.heapify_down(0);
        max
    }

    fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            self.heapify_down(n);
        }
    }

    /// Bottom-up reheapify (swim)
    /// if child is larger than its parent
    /// swap child and its parent recursively (using iteration)
    fn heapify_up(&mut self, index: usize) {
        let mut child = index;
        while child > 0 {
            let parent = (child - 1) / 2;
            if self.data[child] <= self.data[parent] {
                break;
            }
            self.data.swap(child, parent);
            child = parent;
        }
    }

    /// Top-down reheapify (sink)
    /// If the parent is smaller than its child or children
    /// which causes the heap order is violated.
    fn heapify_down(&mut self, index: usize) {
        let mut parent = index;
        while (2 * parent + 1 < self.data.len()) {
            let mut child = 2 * parent + 1;
            if child + 1 < self.data.len() && self.data[child] < self.data[child + 1] {
                child += 1;
            }
            if self.data[parent] >= self.data[child] {
                break;
            }
            self.data.swap(parent, child);
            parent = child;
        }
    }
}

impl<T: Ord + Debug> From<Vec<T>> for BinHeap<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut heap = BinHeap { data: vec };
        heap.rebuild();
        heap
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heap() {
        let arr = vec![0, 2, 7, 8, 1, 2, 3, 4];
        let mut result = BinHeap::from(arr);
        println!("build: {:?}", result);

        result.push(10);
        println!("aft push: {:?}", result);

        let max = result.peek();
        println!("peek max: {:?}", max);

        let val = result.pop();
        println!("pop val: {:?}", val);

        println!("aft pop: {:?}", result);
    }
}
