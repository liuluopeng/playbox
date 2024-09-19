impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut poss_of_piles = vec![];
        for pile in piles {
            let mut poss_of_pile = vec![];

            let mut sum = 0;
            let mut action = 0;
            for coin in pile {
                sum += coin as usize;
                action += 1;
                poss_of_pile.push((action, sum));
            }
            poss_of_piles.push(poss_of_pile);
        }

        let k = k as usize;

        let mut record = vec![vec![0; k + 1]; poss_of_piles.len() + 1];

        let mut setted = vec![vec![false; k + 1]; poss_of_piles.len() + 1];

        Self::find(
            poss_of_piles.len(),
            &mut record,
            &mut setted,
            k,
            &poss_of_piles,
        );

        record[poss_of_piles.len()][k] as i32
    }

    fn find(
        len_limit: usize,
        record: &mut Vec<Vec<usize>>,
        setted: &mut Vec<Vec<bool>>,
        remain_choose_time: usize,
        poss_of_piles: &Vec<Vec<(usize, usize)>>,
    ) -> usize {
        if len_limit == 0 {
            return 0;
        }

        if setted[len_limit][remain_choose_time] == false {
            let mut curr = Self::find(
                len_limit - 1,
                record,
                setted,
                remain_choose_time,
                poss_of_piles,
            );

            for &i in &poss_of_piles[len_limit - 1] {
                let cust = i.0;
                let coin = i.1;

                if remain_choose_time >= cust {
                    let tmp = coin
                        + Self::find(
                            len_limit - 1,
                            record,
                            setted,
                            remain_choose_time - cust,
                            poss_of_piles,
                        );
                    curr = curr.max(tmp);
                }
            }

            setted[len_limit][remain_choose_time] = true;
            record[len_limit][remain_choose_time] = curr;
        }

        record[len_limit][remain_choose_time]
    }
}
use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        let k = 7;
        let piles = vec_2d_leetcode("[[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]]");

        assert_eq!(706, Solution::max_value_of_coins(piles, k));
    }
}
