impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();

        // [ 0..i [0 ... sum/2]]

        // records[i][j]:  左侧长度i的切片 随便选 能达到的集合的sum是j的时候的 
        let mut records = vec![vec![-1; (sum / 2) as usize + 1]; stones.len() + 1];

        // record[i]  左侧随便选的累加和  1 最大 2 小于sum/2
        let mut record = vec![-1; stones.len() + 1];

        Self::find(sum, &mut records, &mut record, stones.len())
    }

    fn find(sum: i32, records: &mut Vec<Vec<i32>>, record: &mut Vec<i32>, len_now: usize) -> i32 {
        

        

    }
}

use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        /*

                示例 1：

        输入：stones = [2,7,4,1,8,1]
        输出：1
        解释：
        组合 2 和 4，得到 2，所以数组转化为 [2,7,1,8,1]，
        组合 7 和 8，得到 1，所以数组转化为 [2,1,1,1]，
        组合 2 和 1，得到 1，所以数组转化为 [1,1,1]，
        组合 1 和 1，得到 0，所以数组转化为 [1]，这就是最优值。
        示例 2：

        输入：stones = [31,26,33,21,40]
        输出：5

                 */

        assert_eq!(1, Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]));
        assert_eq!(5, Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]));
    }
}
