use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; queries.len()];

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut qu_list = vec![];
        for (i, &v) in queries.iter().enumerate() {
            qu_list.push((v, i));
        }

        qu_list.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{:?} {:?}", intervals, qu_list);

        let mut pq: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        let mut idx_now_interval = 0 as usize;

        for q in qu_list {
            let v = q.0;
            let idx_res = q.1;

            while idx_now_interval < intervals.len() && intervals[idx_now_interval][0] <= v {
                // 优先队列存的数据: 区间的(长度, 结束位置)
                pq.push(Reverse((
                    intervals[idx_now_interval][1] - intervals[idx_now_interval][0] + 1,
                    intervals[idx_now_interval][1],
                )));
                idx_now_interval += 1;
            }

            // 删除那些扫描线之前的区间
            while !pq.is_empty() && pq.peek().unwrap().0 .1 < v {
                pq.pop();
            }

            if pq.is_empty() {
                res[idx_res] = -1;
            } else {
                res[idx_res] = pq.peek().unwrap().0 .0;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        leetcode::lc1851::Solution,
        util::{array2d_to_vec2d, vec_2d_leetcode},
    };

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3, 3, 1, 4],
            Solution::min_interval(
                array2d_to_vec2d(&[[1, 4], [2, 4], [3, 6], [4, 4]]),
                vec![2, 3, 4, 5]
            )
        );
    }
}
