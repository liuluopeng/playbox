struct Solution;

impl Solution {
    // k个1-9数字 和是n
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut this_turn = vec![];
        Self::find(&mut res, &mut this_turn, 0, 1, k, n);
        res
    }

    pub fn find(
        res: &mut Vec<Vec<i32>>,
        this_turn: &mut Vec<i32>,
        sum: i32,
        start_num: i32,
        k: i32,
        n: i32,
    ) {
        // println!("{:?}", this_turn);

        if this_turn.len() == k as usize {
            if sum == n {
                res.push(this_turn.to_vec());
            }
            return;
        }

        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in start_num..10 {
            this_turn.push(i);
            Self::find(res, this_turn, sum + i, i + 1, k, n);
            this_turn.pop();
        }
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", Solution::combination_sum3(3, 7));
}
