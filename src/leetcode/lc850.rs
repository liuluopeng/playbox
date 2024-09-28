impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut res: i64 = 0;

        let mut sacn_x_list = vec![];
        for r in rectangles.iter() {
            sacn_x_list.push(r[0]);
            sacn_x_list.push(r[2]);
        }

        sacn_x_list.sort();
        println!("{:?}", sacn_x_list);

        for idx in 0..sacn_x_list.len() - 1 {
            let x_coo1 = sacn_x_list[idx];
            let x_coo2 = sacn_x_list[idx + 1];

            if x_coo2 - x_coo1 == 0 {
                continue;
            }

            let mut y_toge = vec![];
            for r in rectangles.iter() {
                if r[0] <= x_coo1 && x_coo2 <= r[2] {
                    y_toge.push((r[1], r[3]));
                }
            }
            y_toge.sort();

            let mut y_sum = 0 as i64;
            // 求出交集:
            let (mut l, mut r) = (-1, -1);
            for (y_start, y_end) in y_toge {
                if y_start > r {
                    y_sum += (r - l) as i64;
                    l = y_start;
                    r = y_end;
                } else if y_end > r {
                    r = y_end;
                }
            }
            y_sum += (r - l) as i64;
            res += (y_sum * (x_coo2 - x_coo1) as i64) % 1000000007 as i64;
        }

        (res % 1000000007) as i32
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{leetcode::lc850::Solution, util::array2d_to_vec2d};

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            Solution::rectangle_area(array2d_to_vec2d(&[
                [0, 0, 2, 2],
                [1, 0, 2, 3],
                [1, 0, 3, 1]
            ]))
        );
    }
}
