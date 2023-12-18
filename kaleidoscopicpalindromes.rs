use std::io::{self, BufRead};

fn main() {
    let (a, b, k) = get_a_b_k();
    let result = (a..=b)
        .filter(|&n| is_palindrome_in_all_bases(n, k))
        .count();
    println!("{}", result);
}

fn get_a_b_k() -> (i32, i32, i32) {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let abk: Vec<_> = line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let (a, b, k) = (abk[0], abk[1], abk[2]);
    (a, b, k)
}

fn is_palindrome_in_all_bases(num: i32, lim: i32) -> bool {
    for j in 2..=lim {
        if !is_palindrome(num, j) {
            return false;
        }
    }
    true
}

fn is_palindrome(num: i32, base: i32) -> bool {
    let vec = to_base_n(num, base);
    let rev: Vec<_> = vec.iter().rev().cloned().collect();
    let is_palindrome = vec.iter().eq(rev.iter());
    is_palindrome
}

fn to_base_n(mut num: i32, base: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    while num > 0 {
        let digit = num % base;
        result.push(digit);
        num /= base;
    }
    result
}
