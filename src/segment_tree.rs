#[derive(Debug)]
pub struct SegmentTree {
    ori_data: Vec<i32>,
    seg_tree: Vec<i32>,
}
impl SegmentTree {
    pub fn new(ori_data: Vec<i32>) -> SegmentTree {
        let mut seg_tree = vec![0; 4 * ori_data.len()];
        Self::assemble(0, ori_data.len() - 1, 0, &mut seg_tree, &ori_data);
        Self {
            ori_data: ori_data,
            seg_tree: seg_tree,
        }
    }

    fn get_left_child_idx(father_idx: usize) -> usize {
        2 * father_idx + 1
    }
    fn get_right_child_idx(father_idx: usize) -> usize {
        2 * father_idx + 2
    }

    // init [0, len() - 1]
    // 求[l,r]的区域和, 分解:求出[l,mid] [mid+1]的区域和.
    fn assemble(l: usize, r: usize, seg_idx: usize, seg_tree: &mut Vec<i32>, ori_data: &Vec<i32>) {
        if l == r {
            seg_tree[seg_idx] = ori_data[l];
            return;
        }

        //  求出[l,mid] [mid+1]的区域和.
        // l < r
        let mid = (l + r) / 2;

        let left_seg_idx = Self::get_left_child_idx(seg_idx);
        let right_seg_idx = Self::get_right_child_idx(seg_idx);

        Self::assemble(l, mid, left_seg_idx, seg_tree, ori_data);
        Self::assemble(mid + 1, r, right_seg_idx, seg_tree, ori_data);

        seg_tree[seg_idx] = seg_tree[left_seg_idx] + seg_tree[right_seg_idx];
    }

    // query [l,r] 分解成不定的几个
    pub fn query(&self, l: usize, r: usize) -> i32 {
        Self::_query(&self, l, r, 0, self.ori_data.len() - 1, 0)
    }

    // seg_border_l: usize,
    // seg_border_r: usize,
    // seg_idx: usize,
    // 这三个参数确定一个区间
    fn _query(
        &self,
        l: usize,
        r: usize,
        seg_border_l: usize,
        seg_border_r: usize,
        seg_idx: usize,
    ) -> i32 {
        if l == seg_border_l && r == seg_border_r {
            return self.seg_tree[seg_idx];
        }

        let mid = (seg_border_l + seg_border_r) / 2;
        let left_child_idx = Self::get_left_child_idx(seg_idx);
        let right_child_idx = Self::get_right_child_idx(seg_idx);
        // 比较 l r   和  已经分割出来的区间  的 关系:

        if r <= mid {
            //[seg_l   l r    mid     seg_r]                   lr 可能 整体  位于  [seg_l, mid]
            return Self::_query(&self, l, r, seg_border_l, mid, left_child_idx);
        } else if l >= mid + 1 {
            // [seg_l          mid   l   r  seg_r]
            return Self::_query(&self, l, r, mid + 1, seg_border_r, right_child_idx);
        } else {
            // 需要组合两侧的结果  [seg_l   l   mid] [mid+1,  r ,  seg_r]
            let left_res = Self::_query(&self, l, mid, seg_border_l, mid, left_child_idx);
            let right_res = Self::_query(&self, mid + 1, r, mid + 1, seg_border_r, right_child_idx);
            return left_res + right_res;
        }
    }

    // update
    pub fn update(&mut self, idx: usize, new_value: i32) {
        self.ori_data[idx] = new_value;
        let size = self.ori_data.len();
        Self::_update(self, idx, new_value, 0, size - 1, 0);
    }

    fn _update(&mut self, idx: usize, new_value: i32, seg_l: usize, seg_r: usize, seg_idx: usize) {
        if seg_l == seg_r {
            // [seg_l  seg_r]已经逐渐逼近到了需要寻找的位置
            self.seg_tree[seg_idx] = new_value;
            return;
        }

        // seg_l < seg_r
        let mid = (seg_l + seg_r) / 2;
        if idx <= mid {
            Self::_update(
                self,
                idx,
                new_value,
                seg_l,
                mid,
                Self::get_left_child_idx(seg_idx),
            );
        } else {
            // idx >= mid + 1

            Self::_update(
                self,
                idx,
                new_value,
                mid + 1,
                seg_r,
                Self::get_right_child_idx(seg_idx),
            );
        }

        // 更新
        let left_res = self.seg_tree[Self::get_left_child_idx(seg_idx)];
        let right_res = self.seg_tree[Self::get_right_child_idx(seg_idx)];
        self.seg_tree[seg_idx] = left_res + right_res;
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);

        let ori_data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut my_segment = SegmentTree::new(ori_data);

        println!("{:?}", my_segment);

        assert_eq!(21, my_segment.query(0, 6));

        // udpate
        my_segment.update(6, 7);
        // println!("new sum: {:?}", my_segment.query(0, 10));
        assert_eq!(56, my_segment.query(0, 10));
    }
}
