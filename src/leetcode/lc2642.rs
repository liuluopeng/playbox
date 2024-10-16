use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    data: Vec<Vec<(usize, usize)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;

        let mut data = vec![vec![]; n];

        for msg in edges.iter() {
            data[msg[0] as usize].push((msg[1] as usize, msg[2] as usize))
        }

        Self { data }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.data[edge[0] as usize].push((edge[1] as usize, edge[2] as usize))
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut dist = vec![usize::MAX / 2; self.data.len()];
        let mut visited = vec![false; self.data.len()];

        let mut min_heap = BinaryHeap::new();
        dist[node1 as usize] = 0;
        min_heap.push(Reverse((0, node1 as usize)));

        while let Some(Reverse((curr_dis, curr_node))) = min_heap.pop() {
            if visited[curr_node] {
                continue;
            }
            visited[curr_node] = true;
            for &(next_node, next_dist) in self.data[curr_node].iter() {
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
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

struct Solution;
#[cfg(test)]
mod tests {
    use crate::util::array2d_to_vec2d;

    use super::Graph;

    #[test]
    fn it_works() {
        let mut obj = Graph::new(
            4,
            array2d_to_vec2d(&[[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]),
        );

        assert_eq!(6, obj.shortest_path(3, 2));
    }
}
