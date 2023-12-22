use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let n: usize = line.unwrap().parse().unwrap();
        let cs: Vec<_> = lines.by_ref().take(n).map(|l| l.unwrap()).collect();

        let mut S: VecDeque<i32> = VecDeque::new();
        let mut Q: VecDeque<i32> = VecDeque::new();
        let mut P: BinaryHeap<i32> = BinaryHeap::new();
        let mut Sp: bool = true;
        let mut Qp: bool = true;
        let mut Pp: bool = true;
        for c in &cs {
            // println!("{:?}", c);
            let parts: Vec<&str> = c.split_whitespace().collect();
            let t: i32 = parts[0].parse().unwrap();
            let x: i32 = parts[1].parse().unwrap();
            if t == 1 {
                S.push_back(x);
                Q.push_back(x);
                P.push(x);
            } else {
                // t == 2
                match S.pop_back() {
                    Some(v) if v != x => Sp = false,
                    None => Sp = false,
                    _ => {}
                }
                match Q.pop_front() {
                    Some(v) if v != x => Qp = false,
                    None => Qp = false,
                    _ => {}
                }
                match P.pop() {
                    Some(v) if v != x => Pp = false,
                    None => Pp = false,
                    _ => {}
                }
            }
        }
        let sum = Sp as i32 + Qp as i32 + Pp as i32;
        if sum == 0 {
            println!("{}", "impossible");
        } else if sum > 1 {
            println!("{}", "not sure");
        } else if Sp {
            println!("{}", "stack");
        } else if Qp {
            println!("{}", "queue");
        } else if Pp {
            println!("{}", "priority queue");
        } else {
            println!("{}", "oops");
        }
        // println!("{:?}", cs);
    }
}
