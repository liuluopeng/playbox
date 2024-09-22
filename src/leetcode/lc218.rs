use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        let mut ex = HashMap::new();

        for w in banned_words {
            ex.insert(w, true);
        }

        let mut cnt = 0;
        for me in message {
            if ex.contains_key(&me) {
                cnt += 1;
            }
            if cnt >= 2 {
                return true;
            }
        }

        return false;
    }
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        let mut p_queue = BinaryHeap::new();

        let mut watch = vec![];
        for b in &buildings {
            watch.push(b[0]);
            watch.push(b[1]);
        }

        watch.sort();

        let mut now_building = 0;
        for p in watch {
            println!("产生变化的地方: {}", p);

            while now_building < buildings.len() && buildings[now_building][0] <= p
            // && buildings[now_building][1] > p
            {
                p_queue.push((buildings[now_building][2], buildings[now_building][1]));
                now_building += 1;
            }

            while !p_queue.is_empty() && p_queue.peek().unwrap().1 <= p {
                p_queue.pop();
            }

            let mut high = 0;
            if !p_queue.is_empty() {
                high = p_queue.peek().unwrap().0;
            }

            if res.len() > 0 && res.last().unwrap()[1] == high {
                continue;
            }

            res.push(vec![p, high]);
        }

        // 去掉没有变化的部分

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{leetcode::lc218::Solution, util::array2d_to_vec2d};

    #[test]
    fn it_works() {
        assert_eq!(
            array2d_to_vec2d(&[
                [2, 10],
                [3, 15],
                [7, 12],
                [12, 0],
                [15, 10],
                [20, 8],
                [24, 0]
            ]),
            Solution::get_skyline(array2d_to_vec2d(&[
                [2, 9, 10],
                [3, 7, 15],
                [5, 12, 12],
                [15, 20, 10],
                [19, 24, 8]
            ]))
        )
    }
}
