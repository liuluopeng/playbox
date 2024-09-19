

struct Solution;


use std::collections::HashMap;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut m = HashMap::new();
        Self::find(&mut res, vec![], &nums, 0, &mut m);

        res
    }

    pub fn find(
        res: &mut Vec<Vec<i32>>,
        turn: Vec<i32>,
        nums: &Vec<i32>,
        start: usize,
        m: &mut HashMap<Vec<i32>, i32>,
    ) {
        if start == nums.len() {
            let mut turn = turn.clone();
            turn.sort();

            if !m.contains_key(&turn) {
                res.push(turn.clone());
                m.insert(turn, 1);
            }

            return;
        }


        // 选
        let mut turn = turn.clone();
        turn.push(nums[start]);
        Self::find(res, turn.clone(), nums, start + 1, m);
        turn.pop();

        // 不选
        Self::find(res, turn.clone(), nums, start + 1, m);
        turn.pop();
    }
}

fn main() {
    println!("Hello, world!");

    let res = Solution::subsets_with_dup(vec![1, 2, 2]);

    for r in &res {
        println!("{:?}", r)
    }
    println!("{:?}", &res.len());
}
