impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        let mut intervals = intervals.clone();

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut idx = 0;

        while idx < intervals.len() {
            let l = res.len();

            if l > 0 {
                let spring = res[l - 1][1];

                if spring >= intervals[idx][0] {
                    res[l - 1][1] = res[l - 1][1].max(intervals[idx][1]);
                } else {
                    res.push(intervals[idx].clone());
                }
            } else {
                res.push(vec![intervals[0][0], intervals[0][1]]);
            }

            idx += 1;
        }

        res
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::util::array2d_to_vec2d;

    use super::Solution;

    #[test]
    fn it_works() {
        let res = Solution::merge(array2d_to_vec2d(&[[2, 6], [1, 3], [8, 10], [15, 18]]));

        println!("{:?}", res);
    }
}
