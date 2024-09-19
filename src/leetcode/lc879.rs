impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        let mut sum_profit = 0;
        for i in profit {
            sum_profit += i as usize;
        }
        let mut rec = vec![vec![vec![0; n as usize + 1]; sum_profit]; group.len()];
        let mut setted = vec![vec![vec![false; n as usize + 1]; sum_profit]; group.len()];

        Self::find(
            0,
            min_profit as usize,
            n as usize,
            0,
            &group,
            &profit,
            &mut memo,
            &mut rec,
            &mut setted,
        ) as i32
    }

    // 目前,已经累计了sum_profit, 人数的限制还有remain_n. find(idx): group[idx...]  profit[idx...] 的种类个数
    pub fn find(
        sum_profit: usize,
        target_profit: usize,
        remain_n: usize,
        idx_now: usize,
        group: &Vec<i32>,
        profit: &Vec<i32>,
        memo: &mut HashMap<(usize, usize, usize), usize>,
        rec: &mut Vec<Vec<Vec<usize>>>,
        setted: &mut Vec<Vec<Vec<bool>>>,
    ) -> usize {
        let modder = 1000000007;
        // println!("visit: {}", idx_now);

        if idx_now >= group.len() {
            if sum_profit >= target_profit {
                return 1;
            } else {
                return 0;
            }
        }
        if remain_n == 0 {
            if sum_profit >= target_profit {
                return 1;
            } else {
                return 0;
            }
        }

        let mut res = 0;

        if !setted[idx_now][sum_profit][remain_n] {
            // pick
            let mut pick_now = 0;
            if remain_n >= group[idx_now] as usize {
                pick_now = Self::find(
                    sum_profit + profit[idx_now] as usize,
                    target_profit,
                    remain_n - group[idx_now] as usize,
                    idx_now + 1,
                    group,
                    profit,
                    memo,
                    rec,
                    setted,
                );
            }
            // do not pick
            let not_pick_now = Self::find(
                sum_profit,
                target_profit,
                remain_n,
                idx_now + 1,
                group,
                profit,
                memo,
                rec,
                setted,
            );

            res = (pick_now + not_pick_now) % modder;

            // memo.insert((idx_now, sum_profit, remain_n), res);
            setted[idx_now][sum_profit][remain_n] = true;
            rec[idx_now][sum_profit][remain_n] = res;
        } else {
            res = rec[idx_now][sum_profit][remain_n];
        }

          res
    }
}

use std::collections::HashMap;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        // n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
        let n = 10;
        let min_profit = 5;
        let group = vec![2, 3, 5];
        let profit = vec![6, 7, 8];

        assert_eq!(
            7,
            Solution::profitable_schemes(n, min_profit, group, profit)
        );

        let group = vec![
            66, 24, 53, 49, 86, 37, 4, 70, 99, 68, 14, 91, 70, 71, 70, 98, 48, 26, 13, 86, 4, 82,
            1, 7, 51, 37, 27, 87, 2, 65, 93, 66, 99, 28, 17, 93, 83, 91, 45, 3, 59, 87, 92, 62, 77,
            21, 9, 37, 11, 4, 69, 46, 70, 47, 28, 40, 74, 47, 12, 3, 85, 16, 91, 100, 39, 24, 52,
            50, 40, 23, 64, 22, 2, 15, 18, 62, 26, 76, 3, 60, 64, 34, 45, 40, 49, 11, 5, 8, 40, 71,
            12, 60, 3, 51, 31, 5, 42, 52, 15, 36,
        ];
        let profit = vec![
            8, 4, 8, 8, 9, 3, 1, 6, 7, 10, 1, 10, 4, 9, 7, 11, 5, 1, 7, 4, 11, 1, 5, 9, 9, 5, 1,
            10, 0, 10, 4, 1, 1, 1, 6, 9, 3, 6, 2, 5, 4, 7, 8, 5, 2, 3, 0, 6, 4, 5, 9, 9, 10, 7, 1,
            8, 9, 6, 0, 2, 9, 2, 2, 8, 6, 10, 3, 4, 6, 1, 10, 7, 5, 4, 8, 1, 8, 5, 5, 4, 1, 1, 10,
            0, 8, 0, 1, 11, 5, 4, 7, 9, 1, 11, 1, 0, 1, 6, 8, 3,
        ];
        let n = 100;
        let min_profit = 10;

        assert_eq!(
            188883405,
            Solution::profitable_schemes(n, min_profit, group, profit)
        );

        let group = vec![
            24, 23, 7, 4, 26, 3, 7, 11, 1, 7, 1, 3, 5, 26, 26, 1, 13, 12, 2, 1, 7, 4, 1, 27, 13,
            16, 26, 18, 6, 1, 1, 7, 16, 1, 6, 2, 5, 9, 19, 28, 1, 23, 2, 1, 3, 4, 4, 3, 22, 1, 1,
            3, 5, 34, 2, 1, 22, 16, 8, 5, 3, 21, 1, 8, 14, 2, 1, 3, 8, 12, 40, 6, 4, 2, 2, 14, 1,
            11, 9, 1, 7, 1, 1, 1, 6, 6, 4, 1, 1, 7, 8, 10, 20, 2, 14, 31, 1, 13, 1, 9,
        ];
        let profit = vec![
            5, 2, 38, 25, 4, 17, 5, 1, 4, 0, 0, 8, 13, 0, 20, 0, 28, 1, 22, 7, 10, 32, 6, 37, 0,
            11, 6, 11, 23, 20, 13, 13, 6, 2, 36, 1, 0, 9, 4, 5, 6, 14, 20, 1, 13, 6, 33, 0, 22, 1,
            17, 12, 10, 1, 19, 13, 8, 1, 0, 17, 20, 9, 8, 6, 2, 2, 1, 4, 22, 11, 3, 2, 6, 0, 40, 0,
            0, 7, 1, 0, 25, 5, 12, 7, 19, 4, 12, 7, 4, 4, 1, 15, 33, 14, 2, 1, 1, 61, 4, 5,
        ];
        let n = 100;
        let min_profit = 100;

        assert_eq!(
            653387801,
            Solution::profitable_schemes(n, min_profit, group, profit)
        );
    }
}
