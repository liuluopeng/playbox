impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let mut uf = UnionFind::new(nums.len());

        let mut record = HashMap::new();

        //  寻找质因数
        for (idx, &n) in nums.iter().enumerate() {
            let mut ori_n = n;
            let mut n = n;
            let mut fac_try = 2;

            let mut p_list = vec![];

            while fac_try * fac_try <= n {
                if n % fac_try == 0 {
                    // fac_try 是 寻找到的一个结果.
                    p_list.push(fac_try);

                    while n % fac_try == 0 {
                        n /= fac_try;
                    }
                } else {
                    fac_try += 1;
                }
            }
            if n > 1 {
                p_list.push(n);
            }

            for &p in &p_list {
                let arr = record.entry(p).or_insert(vec![]);
                arr.push(idx);
            }

            // println!("origin n : {} {:?}", ori_n, p_list);
        }

        // 相同的质因数, 连接到一个集合里面
        for (k, idx_list) in record {
            // println!("{:?}   k: {}", idx_list, k);

            for i in 0..idx_list.len() - 1 {
                uf.union(idx_list[i], idx_list[i + 1]);
            }
        }

        let set_counter = uf.get_group_counter();

        let res = set_counter.iter().max().copied().unwrap_or_default();

        res as i32
    }
}
use std::collections::HashMap;

use crate::{solution::Solution, union_find::UnionFind};

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            8,
            Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39])
        );
    }
}
