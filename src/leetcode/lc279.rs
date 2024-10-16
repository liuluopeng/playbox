use std::i32;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut pick_list: Vec<usize> = vec![];

        let mut k = 1;
        while k <= (n as f64).sqrt() as usize {
            pick_list.push(k * k);
            k += 1;
        }

        // println!("{:?}", pick_list);

        let mut record = vec![vec![0; n as usize + 1]; pick_list.len() + 1];
        let mut setted = vec![vec![false; n as usize + 1]; pick_list.len() + 1];

        Self::find(
            &mut record,
            &mut setted,
            &pick_list,
            n as usize,
            pick_list.len(),
        )
    }

    fn find(
        record: &mut Vec<Vec<i32>>,
        setted: &mut Vec<Vec<bool>>,
        pick_list: &Vec<usize>,
        remain: usize,
        used_len: usize,
    ) -> i32 {
        if used_len == 0 {
            if remain == 0 {
                return 0;
            } else {
                // still remain
                return i32::MAX;
            }

            // return 1;
        }

        if setted[used_len][remain] {
            return record[used_len][remain];
        }

        // 当前考察的数字是pick_list[i - 1]

        let mut res = i32::MAX;

        // if remain < pick_list[used_len - 1] {
        //     return Self::find(record, setted, pick_list, remain, used_len - 1);
        // }

        // do not pick list[idx]:

        res = res.min(Self::find(record, setted, pick_list, remain, used_len - 1));

        // pick list[idx]
        if remain >= pick_list[used_len - 1] {
            res = res.min(
                1 + Self::find(
                    record,
                    setted,
                    pick_list,
                    remain - pick_list[used_len - 1],
                    used_len,
                ),
            );
        }
        record[used_len][remain] = res;
        setted[used_len][remain] = true;

        record[used_len][remain]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::leetcode::lc279::Solution;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::num_squares(12));

        assert_eq!(1, Solution::num_squares(100));

        assert_eq!(3, Solution::num_squares(2752));
    }
}
