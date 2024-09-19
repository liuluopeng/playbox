impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut res = vec![];

        let mut seg = SegTree::new(&nums1, &nums2);

        for q in queries {
            match q[0] {
                1 => {
                    seg.reverse_nums1(q[1] as usize, q[2] as usize, 0, seg.size - 1, 1);
                }
                2 => {
                    seg.update(q[1] as usize);
                }
                3 => {
                    res.push(seg.get_sum() as i64);
                }
                _ => panic!(),
            }
        }

        res
    }
}

struct SegTree {
    size: usize,

    need_tree: Vec<usize>,

    lazy_rev: Vec<bool>,

    sum_now: usize,
}

impl SegTree {
    pub fn new(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Self {
        let need_tree = vec![0; nums1.len() * 4];
        let lazy_rev = vec![false; nums1.len() * 4];
        let nums2: Vec<usize> = nums2.clone().iter().map(|&n| n as usize).collect();
        let sum_now = nums2.iter().sum();
        let mut seg_tree = SegTree {
            size: nums1.len(),

            need_tree,

            lazy_rev,
            sum_now,
        };

        seg_tree.build2(nums1, 0, seg_tree.size - 1, 1);
        seg_tree
    }

    fn build2(&mut self, need: &Vec<i32>, sl: usize, sr: usize, idx: usize) {
        if sl == sr {
            match need[sl] {
                1 => {
                    self.need_tree[idx] = 1;
                }
                0 => {
                    self.need_tree[idx] = 0;
                }
                _ => {
                    panic!()
                }
            }

            return;
        }

        let mid = (sl + sr) >> 1;
        Self::build2(self, need, sl, mid, idx << 1);
        Self::build2(self, need, mid + 1, sr, idx << 1 | 1);

        self.need_tree[idx] = self.need_tree[idx << 1] + self.need_tree[idx << 1 | 1];
    }

    pub fn reverse_nums1(&mut self, ljob: usize, rjob: usize, sl: usize, sr: usize, idx: usize) {
        if ljob <= sl && sr <= rjob {
            self.lazy_rev[idx] = !self.lazy_rev[idx];
            self.need_tree[idx] = (sr - sl + 1) - self.need_tree[idx];
            return;
        }

        let mid = (sl + sr) >> 1;

        if self.lazy_rev[idx] == true {
            self.lazy_rev[idx << 1] = !self.lazy_rev[idx << 1];
            self.need_tree[idx << 1] = (mid - sl + 1) - self.need_tree[idx << 1];

            self.lazy_rev[idx << 1 | 1] = !self.lazy_rev[idx << 1 | 1];
            self.need_tree[idx << 1 | 1] = (sr - mid) - self.need_tree[idx << 1 | 1];

            self.lazy_rev[idx] = false;
        }

        if ljob <= mid {
            Self::reverse_nums1(self, ljob, rjob, sl, mid, idx << 1);
        }
        if mid + 1 <= rjob {
            Self::reverse_nums1(self, ljob, rjob, mid + 1, sr, idx << 1 | 1);
        }

        self.need_tree[idx] = self.need_tree[idx << 1] + self.need_tree[idx << 1 | 1];
    }

    // 操作类型 2 为 queries[i] = [2, p, 0] 。对于 0 <= i < n 中的所有下标，令 nums2[i] = nums2[i] + nums1[i] * p 。
    // 对于0~segtree.size-1    总的sum的变化是: sum[1] + p *
    pub fn update(&mut self, p: usize) {
        self.sum_now = self.sum_now + p * self.need_tree[1];
    }

    pub fn get_sum(&mut self) -> usize {
        // self.sum_tree[1]
        self.sum_now
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{leetcode::lc2569::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3],
            Solution::handle_query(
                vec![1, 0, 1],
                vec![0, 0, 0],
                vec_2d_leetcode("[[1,1,1],[2,1,0],[3,0,0]]")
            )
        )
    }
}
