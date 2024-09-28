impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let mut known = vec![vec![None; n]; 1 << n];

        Self::find(&mut known, 0, 0, n);

        // for i in 0..n {
        //     println!("{:?}", known[1 << n - 1][i]);
        // }

        // known[1 << n - 1][0].unwrap() as i32

        known[0][0].unwrap() as i32
    }

    pub fn find(
        known: &mut Vec<Vec<Option<usize>>>,
        picked: usize,
        idx: usize,
        n: usize,
    ) -> Option<usize> {
        if idx == n {
            return Some(1);
        }

        let mut res = 0;
        for k in 0..n {
            if picked & (1 << k) == 0 {
                // not pick

                // 把这个数字放在结果的[xxx idx]的位置,看看是否符合要求
                if (k + 1) % (idx + 1) == 0 || (idx + 1) % (k + 1) == 0 {
                    res += Self::find(known, picked | (1 << k), idx + 1, n).unwrap();
                }
            }
        }

        known[picked][idx] = Some(res);

        known[picked][idx]
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc526::Solution;

    #[test]
    fn it_works() {
        /*
           case 1 -> 1;
           case 2 -> 2;
           case 3 -> 3;
           case 4 -> 8;
           case 5 -> 10;
           case 6 -> 36;
           case 7 -> 41;
           case 8 -> 132;
           case 9 -> 250;
           case 10 -> 700;
           case 11 -> 750;
           case 12 -> 4010;
           case 13 -> 4237;
           case 14 -> 10680;
           case 15 -> 24679;
        */
        
        assert_eq!(4010, Solution::count_arrangement(12));
        assert_eq!(750, Solution::count_arrangement(11));
        assert_eq!(700, Solution::count_arrangement(10));
    }
}
