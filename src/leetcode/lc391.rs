// 如果是完美矩形 那么一定满足两点： （1）最左下 最左上 最右下 最右上 的四个点只出现一次 其他点成对出现 （2）四个点围城的矩形面积 = 小矩形的面积之和

use std::{collections::HashMap, i32};

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut varyfi_point = true;

        let mut min_x_coo = i32::MAX;
        let mut min_y_coo = i32::MAX;
        let mut max_x_coo = i32::MIN;
        let mut max_y_coo = i32::MIN;

        let mut small_rect_area_sum = 0;
        let mut point_map = HashMap::new();
        for r in rectangles.iter() {
            let p = point_map.entry((r[0], r[1])).or_insert(0);
            *p += 1;

            let p = point_map.entry((r[2], r[3])).or_insert(0);
            *p += 1;

            let p = point_map.entry((r[0], r[3])).or_insert(0);
            *p += 1;

            let p = point_map.entry((r[2], r[1])).or_insert(0);
            *p += 1;

            small_rect_area_sum += (r[2] - r[0]) * (r[3] - r[1]);

            min_x_coo = r[0].min(r[2]).min(min_x_coo);
            max_x_coo = r[0].max(r[2]).max(max_x_coo);
            min_y_coo = r[1].min(r[3]).min(min_y_coo);
            max_y_coo = r[1].max(r[3]).max(max_y_coo);
        }

        /*

           c     d

           a     b

        */
        let mut abcd = vec![0, 0, 0, 0];

        // for (p, time) in &point_map {
        //     println!("{:?} {:?}", p, time);
        // }

        for (p, time) in point_map {
            // println!("{:?} {:?}", p, time);

            if time == 2 || time == 4 {
                continue;
            } else if time == 1 {
                match p {
                    (a, b) if a == min_x_coo && b == min_y_coo => {
                        abcd[0] += 1;
                    }
                    (a, b) if a == max_x_coo && b == min_y_coo => {
                        abcd[1] += 1;
                    }
                    (a, b) if a == min_x_coo && b == max_y_coo => {
                        abcd[2] += 1;
                    }
                    (a, b) if a == max_x_coo && b == max_y_coo => {
                        abcd[3] += 1;
                    }
                    _ => {
                        varyfi_point = false;
                        break;
                    }
                }
            } else {
                varyfi_point = false;
                break;
            }
        }
        println!(
            "{} {} {} {} {:?}",
            min_x_coo, max_x_coo, min_y_coo, max_y_coo, abcd
        );

        for i in abcd {
            if i != 1 {
                varyfi_point = false;
            }
        }

        (small_rect_area_sum == (max_x_coo - min_x_coo) * (max_y_coo - min_y_coo)) && varyfi_point
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{leetcode::lc391::Solution, util::array2d_to_vec2d};

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_rectangle_cover(array2d_to_vec2d(&[
                [0, 0, 4, 1],
                [7, 0, 8, 2],
                [6, 2, 8, 3],
                [5, 1, 6, 3],
                [4, 0, 5, 1],
                [6, 0, 7, 2],
                [4, 2, 5, 3],
                [2, 1, 4, 3],
                [0, 1, 2, 2],
                [0, 2, 2, 3],
                [4, 1, 5, 2],
                [5, 0, 6, 1]
            ]))
        );
    }
}
