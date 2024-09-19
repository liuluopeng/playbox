impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        //

        let mut record = vec![vec![-2; amount as usize + 1]; coins.len() + 1];

        Self::find(coins.len(), amount as usize, &mut record, &coins);

        let res = record[coins.len()][amount as usize];

        if res != i32::MAX {
            res
        } else {
            -1
        }
    }

    //  使用零钱面额列表的前面的idx个零钱, 兑换出来remain_amount的最少的结果.
    fn find(
        coin_len: usize,
        remain_amount: usize,
        record: &mut Vec<Vec<i32>>,
        coins: &Vec<i32>,
    ) -> i32 {
        if coin_len == 0 {
            // println!("0: {}", remain_amount);

            if remain_amount == 0 {
                record[0][0] = 0;
            } else {
                record[0][remain_amount] = i32::MAX;
            }

            return record[0][remain_amount];
        }

        if record[coin_len][remain_amount] == -2 {
            // 不用当前面额:
            let maybe1 = Self::find(coin_len - 1, remain_amount, record, coins);

            // 使用当前面额:
            let mut maybe2 = i32::MAX;
            if remain_amount >= coins[coin_len - 1] as usize {
                let ok_v = Self::find(
                    coin_len,
                    remain_amount - coins[coin_len - 1] as usize,
                    record,
                    coins,
                );

                if ok_v != i32::MAX { // 我设置i32的最大值是一个到最后有剩余零钱无法兑换的情况. 
                    maybe2 = 1 + ok_v;
                }
            }

            let res = maybe1.min(maybe2);
            record[coin_len][remain_amount] = res;
        }

        record[coin_len][remain_amount]
    }
}

use std::i32;

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));

        assert_eq!(-1, Solution::coin_change(vec![2], 3));

        assert_eq!(0, Solution::coin_change(vec![1], 0));
    }
}
