use std::convert::TryInto;
use std::io;
use std::io::prelude::*;

struct FixedDeque<T> {
    data: Vec<T>,
    start: usize,
    end: usize,
}

impl<T: Clone + Default> FixedDeque<T> {
    fn new(max_size: usize) -> Self {
        FixedDeque {
            data: vec![T::default(); max_size],
            start: max_size / 2 + 1,
            end: max_size / 2,
        }
    }

    fn push_front(&mut self, value: T) {
        self.start -= 1;
        self.data[self.start] = value;
    }

    fn pop_front(&mut self) -> T {
        let value = self.data[self.start].clone();
        self.start += 1;
        value
    }

    fn push_back(&mut self, value: T) {
        self.end += 1;
        self.data[self.end] = value;
    }

    fn pop_back(&mut self) -> T {
        let value = self.data[self.end].clone();
        self.end -= 1;
        value
    }

    fn size(&self) -> usize {
        1 + self.end - self.start
    }
}

struct Teque {
    left: FixedDeque<i32>,
    right: FixedDeque<i32>,
}

impl Teque {
    fn new() -> Self {
        Teque {
            left: FixedDeque::new(2_000_000),
            right: FixedDeque::new(2_000_000),
        }
    }

    fn push_back(&mut self, value: i32) {
        self.right.push_back(value);
        self.balance();
    }

    fn push_front(&mut self, value: i32) {
        self.left.push_front(value);
        self.balance();
    }

    fn push_middle(&mut self, value: i32) {
        self.right.push_front(value);
        self.balance();
    }

    fn get(&self, i: usize) -> i32 {
        let value = if i < self.left.size() {
            let i_left = self.left.start + i;
            self.left.data[i_left]
        } else {
            let i_right = i - self.left.size() + self.right.start;
            self.right.data[i_right]
        };
        value
    }

    fn balance(&mut self) {
        // invariant: 1 >= left.size() - right.size() >= 0
        let left_size = self.left.size() as i32;
        let right_size = self.right.size() as i32;
        if left_size - right_size > 1 {
            self.right.push_front(self.left.pop_back());
        } else if right_size > left_size {
            self.left.push_back(self.right.pop_front());
        }
        /*
        for i in self.left.start..=self.left.end {
            print!("{} ", self.left.data[i]);
        }
        for i in self.right.start..=self.right.end {
            print!("{} ", self.right.data[i]);
        }
        println!()
        */
    }
}

fn main() {
    let mut t = Teque::new();
    let mut lines = io::stdin().lock().lines();
    lines.next();
    for line in lines {
        let line_str = line.unwrap();
        let parts: Vec<_> = line_str.split_whitespace().collect();

        let cmd = parts[0];
        let x = parts[1].parse::<i32>().unwrap();

        match cmd {
            "push_back" => t.push_back(x),
            "push_front" => t.push_front(x),
            "push_middle" => t.push_middle(x),
            "get" => println!("{}", t.get(x.try_into().unwrap())),
            _ => println!("Invalid command: {}", cmd),
        }
    }
}
