use std::collections::BinaryHeap;
use std::io;
use std::io::prelude::*;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            left: BinaryHeap::new(),  // max-heap
            right: BinaryHeap::new(), // min-heap
        }
    }

    fn push(&mut self, x: i32) {
        if self.left.is_empty() || x > *self.left.peek().unwrap() {
            self.right.push(-x);
        } else {
            self.left.push(x);
        }

        // rebalance
        if (self.right.len() as isize) - (self.left.len() as isize) > 1 {
            self.left.push(-self.right.pop().unwrap());
        } else if (self.left.len() as isize) - (self.right.len() as isize) > 1 {
            self.right.push(-self.left.pop().unwrap());
        }

        /*
        println!("{:?}", self.left);
        println!("{:?}", self.right);
        println!(" ");
        */
    }

    fn median(&self) -> i32 {
        let result = if self.left.len() == self.right.len() {
            (self.left.peek().unwrap() - self.right.peek().unwrap()) / 2 // right is negative
        } else if self.right.len() > self.left.len() {
            -*self.right.peek().unwrap()
        } else {
            *self.left.peek().unwrap()
        };
        result
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let T: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..T {
        lines.next(); // skip N
        let xs: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let mut mf = MedianFinder::new();
        let mut sum: i64 = 0;
        for x in &xs {
            mf.push(*x);
            sum += mf.median() as i64;
            // println!("{}", mf.median());
        }
        println!("{}", sum);
    }
}
