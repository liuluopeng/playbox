use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut m = HashMap::new();

        Self::find(&mut res, &mut vec![], 0, &nums, &mut m);

        res
    }

    pub fn find(
        res: &mut Vec<Vec<i32>>,
        turn: &mut Vec<i32>,
        start_index: usize,
        nums: &Vec<i32>,
        m: &mut HashMap<Vec<i32>, bool>,
    ) {
        if turn.len() > 1 {
            if !m.contains_key(turn) {
                res.push(turn.to_vec());
                m.insert(turn.to_vec(), true);
            }
        }

        for i in start_index..nums.len() {

            // 剪枝
            if !turn.is_empty() {
                // turn最后一个元素turn[-1]  和  当前元素nums[i] 比较大小
                // 如果当前元素nums[i] 较小
                // 那么 这一轮i就可以略过。
                if  turn[turn.len() - 1 ] > nums[i] {
                    continue;
                }
            }            

            turn.push(nums[i]);


            Self::find(res, turn, i + 1, nums, m);

            turn.pop();
        }
    }
}

fn main() {
    println!("Hello, world!");
    let nums = Vec::from([4, 4, 3, 2, 1, 9, 9]);

    println!("{:?}", Solution::find_subsequences(nums));
}
