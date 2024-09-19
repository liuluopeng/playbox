impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let all_go_a = costs.iter().map(|x| x[0]).collect::<Vec<i32>>();
        let all_go_a: i32 = all_go_a.iter().sum();

        let mut sub_cost_list = vec![];
        // 如果

        for c in costs {
            sub_cost_list.push(c[1] - c[0]);
        }

        sub_cost_list.sort();

        sub_cost_list.truncate(sub_cost_list.len() / 2);
        let sum_sub_cost: i32 = sub_cost_list.iter().sum();

        all_go_a + sum_sub_cost
    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            110,
            Solution::two_city_sched_cost(vec_2d_leetcode("[[10,20],[30,200],[400,50],[30,20]]"))
        );
    }
}
