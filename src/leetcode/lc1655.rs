use std::collections::HashMap;

impl Solution {
    pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
        let mut cnt_map = HashMap::new();
        for n in nums {
            *cnt_map.entry(n).or_insert(0) += 1;
        }

        let occ: Vec<i32> = cnt_map.keys().cloned().collect();

        quantity.sort_by(|a, b| b.cmp(&a));

        Self::find(&mut vec![], &mut cnt_map, &quantity, 0, &occ)
    }

    pub fn find(
        path: &mut Vec<Vec<i32>>, // 模拟调试用的
        cnt_map: &mut HashMap<i32, i32>,
        quantity: &Vec<i32>, // 从大到小的顾客需求列表
        idx_now: usize,      // 当前的顾客
        occ: &Vec<i32>,      // cnt_map的key列表
    ) -> bool {
        if idx_now == quantity.len() {
            println!("{:?}  {:?}", path, cnt_map);

            return true;
        }

        let mut res = false;
        // 这样可以避免多次借用一个可变的hashmap
        for &oc in occ {
            if let Some(time) = cnt_map.get_mut(&oc) {
                if *time >= quantity[idx_now] {
                    *time -= quantity[idx_now];

                    // 模拟:
                    let used = vec![oc; quantity[idx_now] as usize];
                    path.push(used);

                    res |= Self::find(path, cnt_map, quantity, idx_now + 1, occ);

                    if res {
                        return res;
                    }

                    // 这样恢复:
                    if let Some(rev) = cnt_map.get_mut(&oc) {
                        *rev += quantity[idx_now];
                    }
                    path.pop();
                } else {
                    continue;
                }
            } else {
                panic!("统计错误");
            }
        }

        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc1655::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::can_distribute([1, 1, 2, 2].to_vec(), [2, 2].to_vec())
        );

        assert_eq!(
            false,
            Solution::can_distribute(
                [
                    1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12,
                    13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22,
                    23, 23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32, 32,
                    33, 33, 34, 34, 35, 35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42,
                    43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48, 48, 49, 49, 50, 50
                ]
                .to_vec(),
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 3].to_vec()
            )
        );
    }
}
