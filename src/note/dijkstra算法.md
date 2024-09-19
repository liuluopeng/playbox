
```rust


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

            // if curr_node == node2 as usize {
            //     return curr_dis as i32;
            // }
            visited[curr_node] = true;

            for &(next_node, next_dist) in self.data[curr_node].iter() {
                if visited[next_node] == false {
                    let refresh_dist = dist[curr_node] + next_dist;
                    if dist[next_node] > refresh_dist {
                        dist[next_node] = refresh_dist;

                        min_heap.push(Reverse(( refresh_dist, next_node)));
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
```

```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;
        let mut g = vec![vec![]; n]; // 邻接表
        for t in &times {
            g[t[0] as usize - 1].push((t[1] as usize - 1, t[2]));
        }

        let mut dis = vec![i32::MAX; n];
        dis[k] = 0;
        let mut h = BinaryHeap::new();
        h.push((0, k));
        while let Some((dx, x)) = h.pop() {
            if -dx > dis[x] { // x 之前出堆过
                continue;
            }
            for &(y, d) in &g[x] {
                let new_dis = -dx + d;
                if new_dis < dis[y] {
                    dis[y] = new_dis; // 更新 x 的邻居的最短路
                    h.push((-new_dis, y));
                }
            }
        }
        let mx = *dis.iter().max().unwrap();
        if mx < i32::MAX { mx } else { -1 }
    }
}

作者：灵茶山艾府
链接：https://leetcode.cn/problems/network-delay-time/solutions/2668220/liang-chong-dijkstra-xie-fa-fu-ti-dan-py-ooe8/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
```


1334. 阈值距离内邻居最少的城市 - 力扣（LeetCode） 
https://leetcode.cn/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/ 
