use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            vec![-34, -10, -14, -10, 10],
            Solution::diff_ways_to_compute(String::from("2*3-4*5"))
        );

        assert_eq!(vec![11], Solution::diff_ways_to_compute(String::from("11")));
    }
}

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut res = vec![];

        let exp: Vec<char> = expression.chars().collect();

        let mut record = vec![vec![vec![]; exp.len()]; exp.len()];
        let mut setted = vec![vec![false; exp.len()]; exp.len()];

        let fi = Self::find(&exp, &mut res, 0, exp.len() - 1, &mut record, &mut setted);

        // println!("fi {:?}", fi);

        // for l in record {
        //     println!("{:?}", l);
        // }

        fi
    }

    pub fn find(
        exp: &Vec<char>,
        res: &mut Vec<i32>,
        l: usize,
        r: usize,
        record: &mut Vec<Vec<Vec<i32>>>,
        setted: &mut Vec<Vec<bool>>,
    ) -> Vec<i32> {
        if l == r {
            let num = exp[l] as u8 - '0' as u8;
            let num = num as i32;
            // println!("num: {}", num);
            return vec![num];
        }

        if l + 1 == r {
            let num = (exp[l] as u8 - '0' as u8) as i32 * 10 + (exp[r] as u8 - '0' as u8) as i32;

            return vec![num];
        }

        if setted[l][r] == false {
            // "12"

            //  先做  opr  先做 .  opr是最后一步执行的,两侧已经被执行完成
            let mut opr_idx = l;
            let mut one_res = vec![];

            while opr_idx < exp.len() && opr_idx < r {
                let opr_char = exp[opr_idx];
                if opr_char != '+' && opr_char != '-' && opr_char != '*' {
                    opr_idx += 1;
                    continue;
                }

                let left_part = Self::find(exp, res, l, opr_idx - 1, record, setted);
                let right_part = Self::find(exp, res, opr_idx + 1, r, record, setted);

                match opr_char {
                    '+' => {
                        for &l in &left_part {
                            for &r in &right_part {
                                one_res.push(l + r);
                            }
                        }
                    }

                    '-' => {
                        for &l in &left_part {
                            for &r in &right_part {
                                one_res.push(l - r);
                            }
                        }
                    }

                    '*' => {
                        for &l in &left_part {
                            for &r in &right_part {
                                one_res.push(l * r);
                            }
                        }
                    }

                    _ => {
                        panic!("panic");
                    }
                }

                opr_idx += 1;
            }

            // println!("one result: {:?}", one_res);
            record[l][r] = one_res;

            setted[l][r] = true;
        }

        record[l][r].clone()
    }
}
