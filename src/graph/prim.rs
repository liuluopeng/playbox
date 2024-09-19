use std::{cmp::Reverse, collections::BinaryHeap};

pub fn prim(graph: Vec<Vec<(usize, usize)>>) -> usize {
    let mut sum = 0;

    let mut min_heap = BinaryHeap::new();
    // 随便选择一个节点,比如0
    let mut visited = vec![false; graph.len()];
    visited[0] = true;

    // 初始化, 装载所有的和0相连的边.
    for &(node, weight) in &graph[0] {
        min_heap.push(Reverse((weight, 0, node)));
    }

    while let Some(Reverse((weight, _from_node, dest_node))) = min_heap.pop() {
        if visited[dest_node] {
            continue;
        }

        visited[dest_node] = true;
        sum += weight;

        for &(next_node, next_weight) in &graph[dest_node] {
            if !visited[next_node] {
                min_heap.push(Reverse((next_weight, dest_node, next_node)));
            }
        }
    }

    sum
}
