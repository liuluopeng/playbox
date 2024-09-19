struct Solution;

impl Solution {
    // 返回所有的 1~n 内的k个数字。

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let mut turn = vec![];

        Self::find(&mut res, &mut turn, k, n, 1);

        res
    }

    pub fn find(res: &mut Vec<Vec<i32>>, turn: &mut Vec<i32>, k: i32, n: i32, start_of_i: i32) {
        // println!("结果 {:?} {:?}", turn, start_of_i);

        if (turn.len() == k as usize) {
            res.push(turn.to_vec());
            return;
        }

        for i in start_of_i..n + 1 {
            turn.push(i);

            Self::find(res, turn, k, n, i + 1);

            turn.pop();
        }
    }
}

fn main() {
    println!("{:?}", Solution::combine(4, 2));
}
