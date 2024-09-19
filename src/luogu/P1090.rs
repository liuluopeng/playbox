use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, *},
};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let size: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let second_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let cost = min_merge_cost(parts);
    println!("{}", cost);
}

pub fn min_merge_cost(parts: Vec<usize>) -> usize {
    let mut small_heap = BinaryHeap::new();

    for p in parts {
        small_heap.push(Reverse(p));
    }

    let mut cost = 0;
    //  合并最小的两个.
    while small_heap.len() >= 2 {
        let a = small_heap.pop().unwrap();
        let b = small_heap.pop().unwrap();

        cost += a.0 + b.0;

        small_heap.push(Reverse(a.0 + b.0));
    }

    cost
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::luogu::P1090::min_merge_cost;

    #[test]
    fn it_works() {
        // assert_eq!(15, min_merge_cost(vec![1, 2, 9]));
        main();
    }
}
