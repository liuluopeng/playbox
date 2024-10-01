impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let mut people_list = vec![0; k as usize];

        let mut min_global = i32::MAX;

        // Self::brute(&jobs, 0, &mut people_list, &mut min_global);

        Self::brute2(&jobs, 0, &mut people_list, &mut min_global, 0, 0);

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
    ) {
        if turn_max >= *min_global {
            return;
        }

        if idx == jobs.len() {
            // println!("{:?}", people_list);

            // let mut longest = 0;
            // for &p in people_list.iter() {
            //     longest = longest.max(p);
            // }

            *min_global = turn_max.min(*min_global);
            return;
        }

        if deal_idx < people_list.len() {
            people_list[deal_idx] += jobs[idx];
            Self::brute2(
                jobs,
                idx + 1,
                people_list,
                min_global,
                deal_idx + 1,
                turn_max.max(people_list[deal_idx]),
            );
            people_list[deal_idx] = 0;
        }

        for p in 0..deal_idx {
            people_list[p] += jobs[idx];

            Self::brute2(
                jobs,
                idx + 1,
                people_list,
                min_global,
                deal_idx,
                turn_max.max(people_list[p]),
            );

            people_list[p] -= jobs[idx];
        }
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
