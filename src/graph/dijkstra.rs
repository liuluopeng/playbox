use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: Vec<Vec<(usize, usize)>>, start_node_idx: usize) {
    let mut dist = vec![usize::MAX; graph.len()]; // 节点start到各个节点的距离
    let mut visited = vec![false; graph.len()];

    println!("{:?} {:?} {:?}", graph, dist, visited);

    // 节点start 到  start 距离是0:
    dist[start_node_idx] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((start_node_idx, 0))); // (节点索引, 暂存的距离)

    while !heap.is_empty() {
        let Reverse((node_idx, tmp_dis)) = heap.pop().unwrap();
        println!("pop : {:?} {:?}", node_idx, tmp_dis);

        if visited[node_idx] == true {
            continue;
        } else {
            // visited[node_idx] == false
            visited[node_idx] = true;
            // dist[node_idx] = tmp_dis;
        }

        // 寻找node_idx指向的各个点:
        for &(next_idx, next_dist) in graph[node_idx].iter() {
            if visited[next_idx] == false && dist[node_idx] + next_dist < dist[next_idx] {
                dist[next_idx] = dist[node_idx] + next_dist;
                heap.push(Reverse((next_idx, dist[node_idx] + next_dist)));
            }
        }
    }

    println!("算完的距离  {:?}", dist);
}
#[cfg(test)]
mod tests {

    use crate::util::old_vec_2d_leetcode;

    use super::dijkstra;

    #[test]
    fn it_works() {
        let raw = old_vec_2d_leetcode("[[2,1,1],[2,3,1],[3,4,1]]");
        println!("raw: {:?}", raw);

        let size = 4;
        let mut graph = vec![vec![]; size];

        for edge in raw {
            graph[edge[0] as usize - 1].push((edge[1] as usize - 1, edge[2] as usize));
        }

        dijkstra(graph, 1);
    }
}
