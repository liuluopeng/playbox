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

        // true
    }

    pub fn find(
        path: &mut Vec<Vec<i32>>,
        cnt_map: &mut HashMap<i32, i32>,
        quan: &Vec<i32>,
        done_idx: usize,
        occ: &Vec<i32>,
    ) -> bool {
        if done_idx == quan.len() {
            println!("{:?}  {:?}", path, cnt_map);

            return true;
        }

        let mut res = false;
        // 这样可以避免多次借用一个可变的hashmap
        for &oc in occ {
            if let Some(time) = cnt_map.get_mut(&oc) {
                if *time >= quan[done_idx] {
                    *time -= quan[done_idx];

                    // 模拟:
                    let used = vec![oc; quan[done_idx] as usize];
                    path.push(used);

                    res |= Self::find(path, cnt_map, quan, done_idx + 1, occ);

                    if res {
                        return res;
                    }

                    // 这样恢复:
                    if let Some(rev) = cnt_map.get_mut(&oc) {
                        *rev += quan[done_idx];
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
    #[test]
    fn it_works() {}
}
