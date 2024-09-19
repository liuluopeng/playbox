use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        // assert_eq!(2, Solution::count_eval(String::from("1^0|0|1"), 0));

        assert_eq!(10, Solution::count_eval(String::from("0&0&0&1^1|0"), 1));
    }
}

impl Solution {
    pub fn count_eval(s: String, result: i32) -> i32 {
        let ss: Vec<char> = s.chars().collect();

        let expected = match result {
            1 => true,
            0 => false,
            _ => panic!("fff"),
        };

        let res = 0;

        let mut record = vec![vec![(0, 0); ss.len()]; ss.len()];

        let mut setted = vec![vec![false; ss.len()]; ss.len()];

        // let res = Self::find(&ss, 1, ss.len() - 2, &mut record, &mut setted);
        let res = Self::find(&ss, 0, ss.len() - 1, &mut record, &mut setted);

        if expected == true {
            return res.0 as i32;
        } else {
            return res.1 as i32;
        }
    }

    pub fn find(
        ss: &Vec<char>,
        start_idx: usize,
        end_idx: usize,
        record: &mut Vec<Vec<(usize, usize)>>,
        setted: &mut Vec<Vec<bool>>,
    ) -> (usize, usize) {
        //  & |
        // 1 0 1  =>  2*k_idx and 2*k_idx + 1

        if start_idx == end_idx {
            let my_bool = match ss[start_idx] {
                '1' => {
                    return (1, 0);
                }
                '0' => {
                    return (0, 1);
                }
                _ => {}
            };
        }

        if setted[start_idx][end_idx] == false {
            let mut k_idx = start_idx + 1;

            let mut curr_true = 0;
            let mut curr_false = 0;

            while k_idx < ss.len() && k_idx < end_idx {
                let mut res_true = 0;
                let mut res_false = 0;

                // 用k_idx隔开.
                match ss[k_idx] {
                    '&' => {
                        // 左侧1 的方法      右侧 1 的方法
                        res_true = Self::find(ss, start_idx, k_idx - 1, record, setted).0
                            * Self::find(ss, k_idx + 1, end_idx, record, setted).0;

                        res_false = Self::find(ss, start_idx, k_idx - 1, record, setted).1
                            * Self::find(ss, k_idx + 1, end_idx, record, setted).1
                            + Self::find(ss, start_idx, k_idx - 1, record, setted).1
                                * Self::find(ss, k_idx + 1, end_idx, record, setted).0
                            + Self::find(ss, start_idx, k_idx - 1, record, setted).0
                                * Self::find(ss, k_idx + 1, end_idx, record, setted).1;
                    }

                    '|' => {
                        res_true = Self::find(ss, start_idx, k_idx - 1, record, setted).0
                            * Self::find(ss, k_idx + 1, end_idx, record, setted).0
                            + Self::find(ss, start_idx, k_idx - 1, record, setted).0
                                * Self::find(ss, k_idx + 1, end_idx, record, setted).1
                            + Self::find(ss, start_idx, k_idx - 1, record, setted).1
                                * Self::find(ss, k_idx + 1, end_idx, record, setted).0;

                        res_false = Self::find(ss, start_idx, k_idx - 1, record, setted).1
                            * Self::find(ss, k_idx + 1, end_idx, record, setted).1;
                    }
                    '^' => {
                        // true的情况:  左边true右边false
                        res_true = Self::find(ss, start_idx, k_idx - 1, record, setted).0
                            * Self::find(ss, k_idx + 1, end_idx, record, setted).1
                            + Self::find(ss, start_idx, k_idx - 1, record, setted).1
                                * Self::find(ss, k_idx + 1, end_idx, record, setted).0;

                        // false的情况: 两侧结果一致
                        res_false = Self::find(ss, start_idx, k_idx - 1, record, setted).0
                            * Self::find(ss, k_idx + 1, end_idx, record, setted).0
                            + Self::find(ss, start_idx, k_idx - 1, record, setted).1
                                * Self::find(ss, k_idx + 1, end_idx, record, setted).1;
                    }
                    _ => {
                        println!("panic {:?}", ss[k_idx]);
                        panic!("hhh")
                    }
                }

                k_idx += 2;

                curr_true += res_true;
                curr_false += res_false;
            }

            record[start_idx][end_idx] = (curr_true, curr_false);
            setted[start_idx][end_idx] = true;
        }

        record[start_idx][end_idx]
    }
}
