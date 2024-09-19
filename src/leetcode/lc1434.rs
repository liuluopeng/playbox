use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            24,
            Solution::number_ways(vec_2d_leetcode("[[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]]"))
        );

        assert_eq!(
            842465346, Solution::number_ways(vec_2d_leetcode("[[1,3,5,10,12,13,14,15,16,18,19,20,21,27,34,35,38,39,40],[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40],[3,7,10,12,13,14,15,17,21,25,29,31,35,40],[2,3,7,8,9,11,12,14,15,16,17,18,19,20,22,24,25,28,29,32,33,34,35,36,38],[6,12,17,20,22,26,28,30,31,32,34,35],[1,4,6,7,12,13,14,15,21,22,27,28,30,31,32,35,37,38,40],[6,12,21,25,38],[1,3,4,5,6,7,8,9,10,11,12,13,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,34,35,36,37,38,39,40]]"))
        );
    }
}
use std::collections::HashMap;
impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let size_of_people = hats.len();

        let size_of_hat = 40;

        let mut hat2people = HashMap::new();

        // 构造 idx号帽子: 适合谁谁戴上
        let mut hat_list = vec![vec![]; size_of_hat];

        for i in 0..hats.len() {
            for &hat_idx in &hats[i] {
                // 1 ~ 40
                hat_list[hat_idx as usize - 1].push(i);

                hat2people.insert((hat_idx as usize - 1, i), true);
            }
        }

        println!("{:?}", hat_list); // should be: [0,1,2,3][0,1,2,3]

        let mut record = vec![vec![None; size_of_hat]; (1 << size_of_people)];

        // let mut used_hat_list = vec![false; size_of_hat];

        let res = Self::find(0, 0, &mut record, size_of_people, &hat2people).unwrap();

        (res % 1000000007) as i32
    }

    pub fn find(
        people_pick_status: usize, // 戴帽子的情况的idx
        hat_idx: usize,            // 帽子的index
        record: &mut Vec<Vec<Option<usize>>>,
        size_people: usize,

        h2p: &HashMap<(usize, usize), bool>,
    ) -> Option<usize> {
        if people_pick_status == (1 << size_people) - 1 {
            // println!("pick 全都选完了 {}   hat: {}", people_pick_status, hat_idx);
            return Some(1);
        }

        // 能执行到这里说明下面: 还有人没戴帽子

        if hat_idx >= 40 {
            return Some(0);
        }

        if record[people_pick_status][hat_idx].is_none() {
            let maybe_skip_this_hat =
                Self::find(people_pick_status, hat_idx + 1, record, size_people, h2p).unwrap();

            let mut maybe_use_this_hat = 0;
            // check every body if he like this hat:
            for p_idx in 0..size_people {
                if people_pick_status & (1 << p_idx) == 0 {
                    // 没戴帽子
                    // 没戴帽子 并且 喜欢当前的帽子
                    if h2p.contains_key(&(hat_idx, p_idx)) {
                        maybe_use_this_hat += Self::find(
                            people_pick_status ^ (1 << p_idx),
                            hat_idx + 1,
                            record,
                            size_people,
                            h2p,
                        )
                        .unwrap();
                    }
                }
            }

            let res = (maybe_use_this_hat + maybe_skip_this_hat) % 1000000007;
            record[people_pick_status][hat_idx] = Some(res);
        }

        record[people_pick_status][hat_idx]
    }
}
