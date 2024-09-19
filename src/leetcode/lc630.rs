impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        // 按照规定的最晚日期排序.

        let mut courses = courses.clone();
        courses.sort_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        println!("{:?}", courses);

        let mut now_date = 0;

        let mut heap = BinaryHeap::new();
        for k in courses {
            let deadline = k[1];
            let duration = k[0];

            match now_date + duration <= deadline {
                true => {
                    heap.push(duration);
                    now_date += duration;
                }
                false => {
                    if heap.is_empty() == false {
                        if *heap.peek().unwrap() > duration {
                            heap.push(duration);
                            now_date += (duration - heap.pop().unwrap());
                        }
                    }
                }
            }
        }

        heap.len() as i32
    }
}

use std::collections::BinaryHeap;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::schedule_course(vec_2d_leetcode(
                "[[100,200],[200,1300],[1000,1250],[2000,3200]]"
            ))
        );
    }
}
