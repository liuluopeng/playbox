impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let mut people_list = vec![0; k as usize];

        let mut min_global = i32::MAX;

        // Self::brute(&jobs, 0, &mut people_list, &mut min_global);

        Self::brute2(
            &jobs,
            0,
            &mut people_list,
            &mut min_global,
            0,
            0,
            &mut vec![vec![]; k as usize],
        );

        min_global
    }

    pub fn brute(jobs: &Vec<i32>, idx: usize, people_list: &mut Vec<i32>, min_global: &mut i32) {
        if idx == jobs.len() {
            // println!("{:?}", people_list);

            let mut longest = 0;
            for &p in people_list.iter() {
                longest = longest.max(p);
            }

            *min_global = longest.min(*min_global);
            return;
        }

        for p in 0..people_list.len() {
            people_list[p] += jobs[idx];

            Self::brute(jobs, idx + 1, people_list, min_global);

            people_list[p] -= jobs[idx];
        }
    }

    pub fn brute2(
        jobs: &Vec<i32>,
        idx: usize,
        people_list: &mut Vec<i32>,
        min_global: &mut i32,
        deal_idx: usize, // 加速, 第一轮发牌, 而不是始终给同一个人添加工作
        turn_max: i32,   // 这一轮记录的最长工作时间
        msg: &mut Vec<Vec<i32>>,
    ) {
        // if turn_max >= *min_global {
        //     return;

        // }

        if idx == jobs.len() {
            println!("{:?}", msg);

            // let mut longest = 0;
            // for &p in people_list.iter() {
            //     longest = longest.max(p);
            // }
            // *min_global = longest.min(*min_global);

            *min_global = turn_max.min(*min_global);

            return;
        }

        if deal_idx < people_list.len() {
            people_list[deal_idx] += jobs[idx];

            msg[deal_idx].push(jobs[idx]);
            Self::brute2(
                jobs,
                idx + 1,
                people_list,
                min_global,
                deal_idx + 1,
                turn_max.max(people_list[deal_idx]),
                msg,
            );
            people_list[deal_idx] -= jobs[idx];
            msg[deal_idx].pop();
        }

        // for p in 0..people_list.len() {
        //     // 为什么0..peoplen.len()会超时?
        //     people_list[p] += jobs[idx];

        //     Self::brute2(
        //         jobs,
        //         idx + 1,
        //         people_list,
        //         min_global,
        //         deal_idx,
        //         turn_max.max(people_list[p]),
        //     );

        //     people_list[p] -= jobs[idx];
        // }

        if deal_idx != people_list.len() {
            println!("{:?} deal 不全", msg);
        }

        for p in 0..deal_idx {
            // 为什么0..peoplen.len()会超时?
            people_list[p] += jobs[idx];
            msg[p].push(jobs[idx]);
            Self::brute2(
                jobs,
                idx + 1,
                people_list,
                min_global,
                deal_idx,
                turn_max.max(people_list[p]),
                msg,
            );

            people_list[p] -= jobs[idx];
            msg[p].pop();
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc1723::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            11,
            Solution::minimum_time_required([1, 2, 4, 7, 8].to_vec(), 2)
        );

        // assert_eq!(
        //     504,
        //     Solution::minimum_time_required(
        //         [254, 256, 256, 254, 251, 256, 254, 253, 255, 251, 251, 255].to_vec(),
        //         10
        //     )
        // );

        // assert_eq!(
        //     9899456,
        //     Solution::minimum_time_required(
        //         [
        //             9899456, 8291115, 9477657, 9288480, 5146275, 7697968, 8573153, 3582365,
        //             3758448, 9881935, 2420271, 4542202
        //         ]
        //         .to_vec(),
        //         9
        //     )
        // );
    }
}
