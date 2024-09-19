use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(false, Solution::can_i_win(10, 11));

        assert_eq!(true, Solution::can_i_win(10, 0));

        assert_eq!(true, Solution::can_i_win(10, 1));

        assert_eq!(false, Solution::can_i_win(10, 40));

        assert_eq!(true, Solution::can_i_win(4, 6));

        assert_eq!(false, Solution::can_i_win(20, 210));
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let desired_total = desired_total as usize;
        let max_choosable_integer = max_choosable_integer as usize;

        let mut selected = vec![false; max_choosable_integer + 1];

        if desired_total <= 0 {
            return true;
        }

        let mut record = HashMap::new();

        Self::helper(&mut selected, desired_total, &mut record)
    }

    fn helper(
        selected: &mut Vec<bool>,
        desired_total: usize,
        memo: &mut HashMap<Vec<bool>, bool>,
    ) -> bool {
        if let Some(&result) = memo.get(selected) {
            return result;
        }

        for i in 1..selected.len() {
            if !selected[i] {
                selected[i] = true;
                if desired_total <= i || !Self::helper(selected, desired_total - i, memo) {
                    selected[i] = false;
                    memo.insert(selected.clone(), true);
                    return true;
                }
                selected[i] = false;
            }
        }

        memo.insert(selected.clone(), false);
        false
    }

    fn show_select(selected: &Vec<bool>) -> Vec<usize> {
        let mut arr = vec![];

        for idx in 0..selected.len() {
            if selected[idx] {
                arr.push(idx);
            }
        }

        arr
    }
}
