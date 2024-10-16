use std::{cmp::Reverse, collections::BinaryHeap};

fn shortest_path(graph: &Vec<Vec<(usize, usize)>>, node1: i32, node2: i32) -> i32 {
    let mut dist = vec![usize::MAX / 2; graph.len()];
    let mut visited = vec![false; graph.len()];

    let mut min_heap = BinaryHeap::new();
    dist[node1 as usize] = 0;
    min_heap.push(Reverse((0, node1 as usize)));

    while let Some(Reverse((curr_dis, curr_node))) = min_heap.pop() {
        if visited[curr_node] {
            continue;
        }
        visited[curr_node] = true;
        for &(next_node, next_dist) in &graph[curr_node] {
            if visited[next_node] == false {
                let refresh_dist = dist[curr_node] + next_dist;
                if dist[next_node] > refresh_dist {
                    dist[next_node] = refresh_dist;
                    min_heap.push(Reverse((refresh_dist, next_node)));
                }
            }
        }
    }

    println!("{:?}", dist);

    if dist[node2 as usize] == usize::MAX / 2 {
        -1
    } else {
        dist[node2 as usize] as i32
    }
}
