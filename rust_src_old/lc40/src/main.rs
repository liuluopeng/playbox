struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let mut candidates: Vec<i32> = candidates;
        candidates.sort();

        Solution::find(&candidates, &mut vec![], 0, 0, target, &mut res);

        // println!("{:?}", res);

        res
    }

    // 寻找出一个符合的结果
    // 把所有的组合列举出来, 然后看和是不是target
    pub fn find(
        candidates: &Vec<i32>,
        turn: &mut Vec<i32>,
        sum_now: i32,
        start_index: usize,
        target: i32,
        res: &mut Vec<Vec<i32>>,
    ) {
        if sum_now > target {
            return;
        }

        if sum_now == target {
            // println!("find: {:?}", turn);
            res.push(turn.to_vec());
            return;
        }

        for i in start_index..candidates.len() {
            // 处理这样的一种情况:  选择有  1 1 1 2 2 3 3   有个第一个1就不要后面两个1了.省时间
            if i > start_index && candidates[i] == candidates[i - 1] {
                continue;
            }

            turn.push(candidates[i]);

            Solution::find(
                candidates,
                turn,
                sum_now + candidates[i],
                i + 1,
                target,
                res,
            );
            turn.pop();
        }
    }
}

pub fn main() {
    let can = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let target = 15;

    let can = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;

    let mut r = Solution::combination_sum2(can, target);

    for i in &r {
        println!("{:?}", i);
    }
}
