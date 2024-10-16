use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, PartialEq, PartialOrd)]
struct FloatWrapper(f64);

impl Eq for FloatWrapper {}

impl Ord for FloatWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let n = n as usize;

        let mut graph = vec![vec![]; n];

        for i in 0..edges.len() {
            graph[edges[i][0] as usize].push((edges[i][1] as usize, succ_prob[i]));
            graph[edges[i][1] as usize].push((edges[i][0] as usize, succ_prob[i]));
        }

        let mut prob_list = vec![0.0; n];

        prob_list[start_node as usize] = 1.0;

        let mut max_heap = BinaryHeap::new();

        max_heap.push((FloatWrapper(1.0), start_node as usize));

        let mut visited = vec![false; n];

        while !max_heap.is_empty() {
            let node = max_heap.pop().unwrap();
            visited[node.1] = true;
            for &(b_idx, p) in &graph[node.1] {
                if visited[b_idx] == false && node.0 .0 * p > prob_list[b_idx] {
                    prob_list[b_idx] = node.0 .0 * p;

                    max_heap.push((FloatWrapper(node.0 .0 * p), b_idx));
                }
            }
        }

        let res = prob_list[end_node as usize];
        res
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::{leetcode::lc1514::Solution, util::array2d_to_vec2d};

    #[test]
    fn it_works() {
        assert_eq!(
            0.25,
            Solution::max_probability(
                3,
                array2d_to_vec2d(&[[0, 1], [1, 2], [0, 2]]),
                [0.5, 0.5, 0.2].to_vec(),
                0,
                2
            )
        );

        assert_eq!(
            0.2139,
            Solution::max_probability(
                5,
                array2d_to_vec2d(&[[1, 4], [2, 4], [0, 4], [0, 3], [0, 2], [2, 3]]),
                [0.37, 0.17, 0.93, 0.23, 0.39, 0.04].to_vec(),
                3,
                4
            )
        );
    }
}
