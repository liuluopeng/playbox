// 所有边入堆,
// 最短边 出堆.   使用并查集检测边的端点是否属于同一个集合.(检测环路)

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::union_find::UnionFind;

pub fn kruskal(graph: Vec<Vec<(usize, usize)>>) -> usize {
    let mut sum_weight = 0;

    let mut union_find = UnionFind::new(graph.len());

    let mut min_heap = BinaryHeap::new();

    for (node_idx, edges) in graph.iter().enumerate() {
        for &(dest_node, weight) in edges {
            min_heap.push(Reverse((weight, node_idx, dest_node)));
        }
    }

    let mut counter = 0; // 切分的当前已经纳入的结点
    while counter != graph.len() && !min_heap.is_empty() {
        let Reverse((weight, from, dest)) = min_heap.pop().unwrap();

        // 把from  dest合并到同一个集合

        if !union_find.is_connected(from, dest) {
            union_find.union(from, dest);
            sum_weight += weight;
            counter += 1;
        } else {
            continue;
        }
    }

    sum_weight
}
