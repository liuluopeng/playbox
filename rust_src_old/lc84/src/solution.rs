use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::TreeNode;

pub struct Solution;

// write my solution here:
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut res = vec![];
        let mut st = vec![];
        let mut left_msg = vec![];
        let mut right_msg = vec![];

        for (i, hei) in heights.iter().enumerate() {
            while !st.is_empty() && heights[st[st.len() - 1]] >= *hei {
                st.pop();
            }

            if st.is_empty() {
                left_msg.push(0);
            } else {
                left_msg.push(st[st.len() - 1] + 1);
            }
            st.push(i);
        }

        st.clear();
        for (i, hei) in heights.iter().enumerate().rev() {
            while !st.is_empty() && *hei <= heights[st[st.len() - 1]] {
                st.pop();
            }

            if st.is_empty() {
                right_msg.push(heights.len() - 1);
            } else {
                right_msg.push(st[st.len() - 1] - 1);
            }

            st.push(i);
        }
        right_msg.reverse();

        println!("{:?} left \n{:?} right", left_msg, right_msg);

        for (i, hei) in heights.iter().enumerate() {
            let mut try_area = 0;

            // let mut peek_left = if left_msg[i] == 0 {
            //     -1
            // } else {
            //     left_msg[i] as i32
            // };

            let mut peek_left = left_msg[i];

            let mut peek_right = right_msg[i];

            let area = (peek_right as i32 - peek_left as i32 + 1) as i32 * heights[i];

            res.push(area);
        }

        println!("{:?}", res);

        let mut m = res[0];
        for i in res.iter() {
            m = m.max(*i);
        }
        m
    }
}
/// 占位 用来引入需要的函数
pub struct Solution2;
impl Solution2 {
    pub fn ppp(root: Option<Rc<RefCell<TreeNode>>>) {}
}

#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let st = "[1,2,3]";
        let root = string2tree(st);

        let arr = vec![2, 1, 5, 6, 2, 3];
        let arr = vec![15, 6, 15, 7, 16, 8, 7, 0];
        // let arr = vec!
        println!("{}", Solution::largest_rectangle_area(arr));
    }
}
