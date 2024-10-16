use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut graph = vec![vec![]; n];
        for r in roads {
            let a = r[0] as usize;
            let b = r[1] as usize;
            let l = r[2] as usize;

            graph[a].push((b, l));
            graph[b].push((a, l));
        }

        let mut max_heap = BinaryHeap::new();

        // shortest[i]: 0-i的最短距离
        let mut shortest = vec![usize::MAX / 2; n];
        shortest[0] = 0;
        // dp[i]: 0~i的最短路 的 不同路径个数.
        let mut dp = vec![0; n];
        dp[0] = 1;

        max_heap.push(Reverse((0, 0)));
        while !max_heap.is_empty() {
            let Reverse((accu_length, begin_idx)) = max_heap.pop().unwrap();

            for &(next_idx, add_length) in &graph[begin_idx] {  // 延伸原本的最短路径, 
                if accu_length + add_length < shortest[next_idx] {
                    shortest[next_idx] = accu_length + add_length;
                    max_heap.push(Reverse((accu_length + add_length, next_idx)));
                    dp[next_idx] = dp[begin_idx];
                } else if accu_length + add_length == shortest[next_idx] { // 发现一条新的路径
                    dp[next_idx] = (dp[next_idx] + dp[begin_idx]) % 1000000007;
                }
            }
        }

        println!("起点到终点  最短距离: {}", shortest[n - 1]);

        dp[n - 1]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::{leetcode::lc1976::Solution, util::array2d_to_vec2d};

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            Solution::count_paths(
                7,
                array2d_to_vec2d(&[
                    [0, 6, 7],
                    [0, 1, 2],
                    [1, 2, 3],
                    [1, 3, 3],
                    [6, 3, 3],
                    [3, 5, 1],
                    [6, 5, 1],
                    [2, 5, 1],
                    [0, 4, 5],
                    [4, 6, 2]
                ])
            )
        );
    }
}
