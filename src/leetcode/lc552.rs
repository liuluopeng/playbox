impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mut res = 0;
        let n: usize = n as usize;

        // n:   abs: 0 1 late:0 1 2
        let mut record = vec![vec![vec![0; 3]; 2]; n];

        // A 可以是: 0 1
        // 连续的L 可以是  0 1 2
        // res += Self::find(n, 0, 0, &mut record);
        // res += Self::find(n, 0, 1, &mut record);
        // res += Self::find(n, 0, 2, &mut record);
        // res += Self::find(n, 1, 0, &mut record);
        // res += Self::find(n, 1, 1, &mut record);
        // res += Self::find(n, 1, 2, &mut record);
        // res as i32

        res = Self::dfs(0, n, 0, 0, &mut record);

        (res % 1000000007) as i32
    }

    pub fn dfs(
        len: usize,
        size: usize,
        absent: usize,
        late: usize,
        record: &mut Vec<Vec<Vec<usize>>>,
    ) -> usize {
        if len >= size {
            return 1;
        }

        if record[len][absent][late] == 0 {
            let mut res = 0;
            res = Self::dfs(len + 1, size, absent, 0, record);
            res %= 1000000007;

            if absent < 1 {
                res += Self::dfs(len + 1, size, 1, 0, record);
                res %= 1000000007;
            }
            if late < 2 {
                res += Self::dfs(len + 1, size, absent, late + 1, record);
                res %= 1000000007;
            }

            res %= 1000000007;
            record[len][absent][late] = res;
        }

        record[len][absent][late]
    }

    pub fn find(
        accu_len: usize,
        absent_total_now: usize,         // 前len个里面有这么多缺勤
        last_late_continuous_now: usize, // 前len个里面的最后几个是连续这么多的迟到
        record: &mut Vec<Vec<usize>>,
    ) -> usize {
        // 路径 需要 注意:   Absent的总数.     Late的连续的情况.

        if accu_len == 0 {
            return 1;
        }

        let mut res = 0;
        let new_len = accu_len - 1;
        match (absent_total_now, last_late_continuous_now) {
            (0, 0) => {
                res += Self::find(new_len, 0, 0, record);
                res += Self::find(new_len, 0, 1, record);
                res += Self::find(new_len, 0, 2, record);
                res += Self::find(new_len, 1, 0, record);
                res += Self::find(new_len, 1, 1, record);
                res += Self::find(new_len, 1, 2, record);
            }
            (0, 1) => {
                res += Self::find(new_len, 0, 0, record);
                res += Self::find(new_len, 0, 1, record);

                res += Self::find(new_len, 1, 0, record);
                res += Self::find(new_len, 1, 1, record);
            }
            (0, 2) => {
                res += Self::find(new_len, 0, 0, record);

                res += Self::find(new_len, 1, 0, record);
            }
            (1, 0) => {
                res += Self::find(new_len, 0, 0, record);
                res += Self::find(new_len, 0, 1, record);
                res += Self::find(new_len, 0, 2, record);
            }
            (1, 1) => {
                res += Self::find(new_len, 0, 0, record);
                res += Self::find(new_len, 0, 1, record);
            }
            (1, 2) => {
                res += Self::find(new_len, 0, 0, record);
            }

            _ => {
                panic!("other case");
                return 0;
            }
        }

        res
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(8, Solution::check_record(2));
        assert_eq!(183236316, Solution::check_record(10101));
    }
}
