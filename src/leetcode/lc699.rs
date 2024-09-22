#[cfg(test)]
mod tests {
    use crate::{leetcode::lc699::Solution, util::old_vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            vec![2, 5, 5],
            Solution::falling_squares(old_vec_2d_leetcode("[[1,2],[2,3],[6,1]]"))
        );

        assert_eq!(
            vec![100, 100],
            Solution::falling_squares(old_vec_2d_leetcode("[[100,100],[200,100]]"))
        );
    }
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];

        // 离散化: 找出需要的数据: 比如[1,100]中间的数字没用, 可以变成[1,2]

        let mut exist: Vec<i32> = vec![];
        for p in positions.iter() {
            let st_idx = p[0];
            let end_idx = st_idx + p[1] - 1;

            exist.push(st_idx);
            exist.push(end_idx);
        }

        let mut exist = exist
            .into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();

        exist.sort();
        println!("after set {:?}", exist);
        let mut quick_find = HashMap::new();
        for (i, &v) in exist.iter().enumerate() {
            quick_find.insert(v, i);
        }

        let mut data = vec![0; exist.len()];

        let mut seg = SegTree::new(&data);

        for p in positions.iter() {
            // 寻找start end对应的小数字的代表.

            let resp_start_idx = *quick_find.get(&p[0]).unwrap();
            let resp_end_idx = *quick_find.get(&(p[0] + p[1] - 1)).unwrap();

            let ori_height = seg.query_max(resp_start_idx, resp_end_idx, 0, seg.size - 1, 1);

            let new_height = ori_height + p[1] as usize;

            // 第一次: 修改每一个影响到的点:
            // for k in resp_start_idx..=resp_end_idx {
            //     seg.sigle_update(k, new_height, 0, seg.size - 1, 1);
            // }

            // 第二次改进:  lazy pa
            seg.update(resp_start_idx, resp_end_idx, new_height, 0, seg.size - 1, 1);

            // println!(
            //     "{:?} {:?} ori:{:?} add {:?}   {:?}",
            //     resp_start_idx, resp_end_idx, ori_height, p[1], seg.tree
            // );

            // 当前最高:
            res.push(seg.query_max(0, data.len() - 1, 0, data.len() - 1, 1) as i32);
        }

        res
    }
}

struct SegTree {
    tree: Vec<usize>,
    size: usize,
    reset_lazy: Vec<Option<usize>>,
}

impl SegTree {
    pub fn new(data: &Vec<i32>) -> Self {
        let mut tree = vec![0; data.len() * 4];
        let mut size = data.len();
        let reset_lazy = vec![None; data.len() * 4];

        Self::build(0, size - 1, 1, data, &mut tree);

        let mut seg_tree = SegTree {
            tree,
            size,
            reset_lazy,
        };

        seg_tree
    }

    fn build(l: usize, r: usize, idx_tree: usize, data: &Vec<i32>, tree: &mut Vec<usize>) {
        if l == r {
            tree[idx_tree] = data[l] as usize;
            return;
        }

        let mid = (l + r) / 2;
        Self::build(l, mid, idx_tree * 2, data, tree);
        Self::build(mid + 1, r, idx_tree, data, tree);

        tree[idx_tree] = tree[idx_tree * 2].max(tree[idx_tree * 2 + 1]);
    }

    pub fn query_max(
        &mut self,
        l: usize,
        r: usize,
        sub_l: usize,
        sub_r: usize,
        idx: usize,
    ) -> usize {
        // 如果出现[l  sub_l  sub_r  r]这种情况, 可以提前知道一部分结果
        if l <= sub_l && sub_r <= r {
            return self.tree[idx];
        }
        Self::down(self, sub_l, sub_r, idx);

        let sub_mid = (sub_l + sub_r) / 2;

        let mut max = usize::MIN;

        // 如果[l..sub_mid]是存在的, 那么需要看看左侧.
        if sub_mid >= l {
            max = max.max(Self::query_max(self, l, r, sub_l, sub_mid, idx * 2));
        }

        // 如果[sub_mid+1..r]不是空集, 那么需要看看右侧
        if sub_mid + 1 <= r {
            max = max.max(Self::query_max(self, l, r, sub_mid + 1, sub_r, idx * 2 + 1));
        }

        max
    }

    // 更新区间[l..r]内的每一个数值
    pub fn update(
        &mut self,
        l: usize,
        r: usize,
        new_value: usize,
        sub_l: usize,
        sub_r: usize,
        idx: usize,
    ) {
        if l <= sub_l && sub_r <= r {
            self.tree[idx] = new_value;
            self.reset_lazy[idx] = Some(new_value);
            return;
        } else {
            let sub_mid = (sub_l + sub_r) / 2;

            Self::down(self, sub_l, sub_r, idx);

            if l <= sub_mid {
                Self::update(self, l, r, new_value, sub_l, sub_mid, idx * 2);
            }
            if sub_mid + 1 <= r {
                Self::update(self, l, r, new_value, sub_mid + 1, sub_r, idx * 2 + 1);
            }

            self.tree[idx] = self.tree[idx * 2].max(self.tree[idx * 2 + 1]);
        }
    }

    pub fn down(&mut self, sub_l: usize, sub_r: usize, idx: usize) {
        if sub_l == sub_r {
            return;
        }

        if self.reset_lazy[idx].is_some() {
            let new_value = self.reset_lazy[idx].unwrap();
            self.tree[idx * 2] = new_value;
            self.tree[idx * 2 + 1] = new_value;

            self.reset_lazy[idx * 2] = Some(new_value);
            self.reset_lazy[idx * 2 + 1] = Some(new_value);

            self.reset_lazy[idx] = None;
        }
    }

    // 单点更新
    pub fn sigle_update(
        &mut self,
        target_idx: usize,
        new_value: usize,
        sub_l: usize,
        sub_r: usize,
        idx: usize,
    ) {
        if sub_l == sub_r && sub_l == target_idx {
            self.tree[idx] = new_value;
            return;
        }
        let sub_mid = (sub_l + sub_r) / 2;

        if target_idx <= sub_mid {
            Self::sigle_update(self, target_idx, new_value, sub_l, sub_mid, idx * 2);
        } else {
            Self::sigle_update(self, target_idx, new_value, sub_mid + 1, sub_r, idx * 2 + 1);
        }

        self.tree[idx] = self.tree[idx * 2].max(self.tree[idx * 2 + 1]);
    }
}
