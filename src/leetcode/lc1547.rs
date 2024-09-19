impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts: Vec<usize> = cuts.iter().map(|&x| x as usize).collect();

        cuts.push(0);
        cuts.push(n as usize);

        cuts.sort();

        let mut record = vec![vec![0; cuts.len()]; cuts.len()];
        let mut setted = vec![vec![false; cuts.len()]; cuts.len()];

        let mut cutted = vec![false; cuts.len()];

        println!("{:?}", cuts);

        Self::find(
            &cuts,
            &mut record,
            &mut setted,
            0,
            cuts.len() - 1,
            &mut cutted,
            0,
            n as usize,
        ) as i32
    }

    pub fn find(
        cuts: &Vec<usize>,
        record: &mut Vec<Vec<usize>>,
        setted: &mut Vec<Vec<bool>>,
        start_idx: usize,
        end_idx: usize,
        cutted: &mut Vec<bool>,

        stick_start: usize,
        stick_end: usize,
    ) -> usize {
        // let my_len = stick_end - stick_start;

        if start_idx >= end_idx {
            return 0;
        }

        if cuts[start_idx] + 1 == cuts[end_idx] {
            return 0;
        }

        if setted[start_idx][end_idx] == false {
            let my_len = cuts[end_idx] - cuts[start_idx];

            let mut res = usize::MAX;

            // 还有可能切一刀吗?
            for k in start_idx + 1..end_idx {
                res = res.min(
                    my_len
                        + Self::find(
                            cuts,
                            record,
                            setted,
                            start_idx,
                            k,
                            cutted,
                            stick_start,
                            stick_end,
                        )
                        + Self::find(
                            cuts,
                            record,
                            setted,
                            k,
                            end_idx,
                            cutted,
                            stick_start,
                            stick_end,
                        ),
                );
            }

            if res == usize::MAX {
                res = 0;
            }

            record[start_idx][end_idx] = res;
            setted[start_idx][end_idx] = true;
        }

        record[start_idx][end_idx]
    }
}
use std::usize;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(16, Solution::min_cost(7, vec![1, 3, 4, 5]));

        assert_eq!(
            127,
            Solution::min_cost(
                30,
                vec![13, 25, 16, 20, 26, 5, 27, 8, 23, 14, 6, 15, 21, 24, 29, 1, 19, 9, 3]
            )
        );

        assert_eq!(
            47433,
            Solution::min_cost(
                7623,
                vec![
                    3611, 303, 6831, 2410, 1176, 461, 7059, 5534, 7215, 2563, 2875, 6178, 588, 70,
                    3582, 4278, 5138, 7028, 5744, 4326, 7282, 7312, 4577, 1133, 6780, 197, 2950,
                    6989, 5015, 5158, 3490, 1270, 6709, 6675, 5076, 3439, 6378, 2800, 5618, 4098,
                    3253, 911, 5432, 7231, 6029, 652, 669, 5183, 7586, 1942, 5642, 3693, 5754, 807,
                    2428, 5872, 3640, 5709, 817, 603, 6205, 6787, 2444, 4287, 5841, 5735, 893,
                    4849, 2258, 3874, 2732, 4122, 1741, 2774, 3565, 1525, 3039, 3891, 6491, 2921,
                    1716, 446, 6948, 4145, 1889, 3657, 4695, 2955, 6007, 1979, 121, 7572, 3330,
                    3508
                ]
            )
        );

        assert_eq!(
            35316,
            Solution::min_cost(
                10000,
                vec![6816, 8372, 3750, 3661, 9442, 4813, 1507, 1421, 5448, 7405, 2401]
            )
        );
    }
}
