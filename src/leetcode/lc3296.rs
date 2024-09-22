use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut res = 0;

        let mut min_heap = BinaryHeap::new();

        let mut remain = mountain_height;
        for &w in worker_times.iter() {
            // 需要知道: 这个人积累的时间 这个人现在需要的时间
            min_heap.push(Reverse((w as i64, 1 as i64, w as i64)));
        }

        while remain > 0 {
            if !min_heap.is_empty() {
                let m = min_heap.pop().unwrap();
                remain -= 1;
                res = res.max(m.0 .0);

                let new_t = m.0 .1 + 1;
                let new_w = m.0 .0 + new_t * m.0 .2;
                let unit_w = m.0 .2;

                min_heap.push(Reverse((new_w, new_t, unit_w)));
            }
        }

        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc3296::Solution;

    #[test]
    fn it_works() {
        assert_eq!(12, Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]));
        assert_eq!(12, Solution::min_number_of_seconds(100000, vec![1]));
    }
}
