struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        Self::find(candidates, &mut vec![], &mut 0, target, &mut res);
        res
    }

    pub fn find(
        candidates: Vec<i32>,
        turn: &mut Vec<i32>,
        index_candi: &mut usize,
        target: i32,
        res: &mut Vec<Vec<i32>>,
    ) {
        if Self::sum(turn.to_vec()) >= target {
            if Self::sum(turn.to_vec()) == target {
                res.push(turn.to_vec());
            }
            return;
        }

        for mut i in (*index_candi as usize)..candidates.len() {
            turn.push(candidates[i]);
            Self::find(candidates.clone(), turn, &mut i, target, res);
            turn.pop();
        }
    }

    pub fn sum(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in arr {
            sum += i;
        }
        sum
    }
}

fn main() {
    println!("Hello, world!");

    let res = Solution::combination_sum(vec![2, 3, 6, 7], 7);
    println!("{:?}", res);
}
