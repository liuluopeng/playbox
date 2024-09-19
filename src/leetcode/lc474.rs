impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        // 最多 有 m 个 0 和 n 个 1

        let (m, n) = (m as usize, n as usize);
        let mut memo = HashMap::new();

        Self::find(&strs, 0, m, n, &mut memo) as i32
    }

    // strs[i]开始到结束, 满足条件的最大集合个数
    fn find(
        strs: &Vec<String>,
        now_idx: usize,
        remain_0: usize,
        reamain_1: usize,
        memo: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if now_idx >= strs.len() {
            return 0;
        }

        if !memo.contains_key(&(now_idx, remain_0, reamain_1)) {
            let (ones, zeros) = Self::one_zero(&strs[now_idx]);

            // 不选择strs[i]
            let res1 = Self::find(strs, now_idx + 1, remain_0, reamain_1, memo);

            // 选择strs[i]

            let mut res2 = 0;
            if remain_0 >= zeros && reamain_1 >= ones {
                res2 = 1 + Self::find(strs, now_idx + 1, remain_0 - zeros, reamain_1 - ones, memo);
            }

            memo.insert((now_idx, remain_0, reamain_1), res1.max(res2));
        }

        *memo.get(&(now_idx, remain_0, reamain_1)).unwrap()
    }

    fn one_zero(s: &String) -> (usize, usize) {
        let mut ones = 0;

        for c in s.chars() {
            if c == '1' {
                ones += 1;
            }
        }

        (ones, s.len() - ones)
    }
}

use std::collections::HashMap;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        let m = 5;
        let n = 3;

        // ["10","0001","111001","1","0"]
        let strs = vec![
            String::from("10"),
            String::from("0001"),
            String::from("111001"),
            String::from("1"),
            String::from("0"),
        ];

        assert_eq!(4, Solution::find_max_form(strs, m, n));

        // 原始字符串数组
        let str_array = [
            "11", "11", "0", "0", "10", "1", "1", "0", "11", "1", "0", "111", "11111000", "0",
            "11", "000", "1", "1", "0", "00", "1", "101", "001", "000", "0", "00", "0011", "0",
            "10000",
        ];

        // 将字符串数组转换为 Vec<String>
        let strs: Vec<String> = str_array.iter().map(|&s| s.to_string()).collect();

        let m = 90;
        let n = 66;
        assert_eq!(29, Solution::find_max_form(strs, m, n));
    }
}
