use std::collections::HashMap;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(true, Solution::makesquare(vec![1, 1, 2, 2, 2]));

        assert_eq!(false, Solution::makesquare(vec![3, 3, 3, 3, 4]));

        assert_eq!(
            true,
            Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3])
        );
    }
}

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();

        if sum % 4 != 0 {
            return false;
        }

        let edge_length = sum / 4;

        let mut pick = vec![false; matchsticks.len()];

        let mut uesd_counter = 0;

        let mut record = HashMap::new();

        Self::find(
            edge_length,
            &mut pick,
            &matchsticks,
            0,
            &mut uesd_counter,
            &mut record,
        )
    }

    // 使用map作为备忘录:
    // 尝试 每条边 达到 edge length
    fn find(
        edge_length: i32,
        pick: &mut Vec<bool>,
        stick: &Vec<i32>,
        curr_edge_now: i32,
        used_counter: &mut usize,
        record: &mut HashMap<Vec<bool>, bool>,
    ) -> bool {
        // println!("{:?}", *used_counter);

        if record.contains_key(pick) {
            return *record.get(pick).unwrap();
        }

        if *used_counter == pick.len() {
            record.insert(pick.to_vec(), true);
            return true;
        }

        let mut res = false;
        for idx in 0..pick.len() {
            if pick[idx] == false {
                if curr_edge_now + stick[idx] < edge_length {
                    pick[idx] = true;
                    *used_counter += 1;
                    res |= Self::find(
                        edge_length,
                        pick,
                        stick,
                        curr_edge_now + stick[idx],
                        used_counter,
                        record,
                    );
                    pick[idx] = false;
                    *used_counter -= 1;
                } else if curr_edge_now + stick[idx] == edge_length {
                    pick[idx] = true;
                    *used_counter += 1;
                    res |= Self::find(edge_length, pick, stick, 0, used_counter, record);
                    pick[idx] = false;
                    *used_counter -= 1;
                } else {
                    continue; // 这个火柴太大了,跳过这个火柴
                }
            }
        }

        record.insert(pick.to_vec(), res);
        res
    }
}
