struct CountIntervals {
    tree: Vec<usize>,
    left: Vec<Option<usize>>,
    right: Vec<Option<usize>>,
    // cnt: usize,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    fn new() -> Self {
        let cnt_interval = Self {
            tree: vec![0],
            left: vec![],
            right: vec![],
            // cnt: 1, // 当前的
            size: 1_000_000_001,
            // size: 1_001,
        };

        cnt_interval
    }

    fn add(&mut self, left: i32, right: i32) {
        Self::_add(
            self,
            left as usize - 1,
            right as usize - 1,
            0,
            self.size - 1,
            0,
        );

        // 默认状态是: tree[0] = 0 代表 区间[1..100000] 没有标记的数字; 没有左孩子[0..50000]和右区间[50001..100000]
    }

    fn _add(&mut self, ljob: usize, rjob: usize, sl: usize, sr: usize, idx: usize) {
        // println!(
        //     "sum:{:?}  left{:?} right:{:?}  idx:{:?} resp:{}~{}",
        //     self.tree, self.left, self.right, idx, sl, sr
        // );

        if idx >= self.left.len() {
            self.left.push(None);
            self.right.push(None);
        }

        if ljob <= sl && sr <= rjob {
            self.tree[idx] = self.tree[idx].max(sr - sl + 1);
            return;
        }

        let mid = (sl + sr) >> 1;

        if ljob <= mid {
            if self.left[idx].is_none() {
                // println!("{:?} resp {}~{} 没有left", idx, sl, sr);
                self.tree.push(0);
                self.left[idx] = Some(self.tree.len() - 1);
            }

            Self::_add(self, ljob, rjob, sl, mid, self.left[idx].unwrap());
        }

        if mid + 1 <= rjob {
            if self.right[idx].is_none() {
                // println!("{:?} resp {}~{} 没有right ", idx, sl, sr);
                self.tree.push(0);
                self.right[idx] = Some(self.tree.len() - 1);
                // println!("new: self.right   {:?}", self.right[idx]);
            }
            Self::_add(self, ljob, rjob, mid + 1, sr, self.right[idx].unwrap());
        }

        let mut sum = 0;
        if let Some(left_idx) = self.left[idx] {
            sum += self.tree[left_idx];
        }
        if let Some(right_idx) = self.right[idx] {
            sum += self.tree[right_idx];
        }

        self.tree[idx] = self.tree[idx].max(sum);

        // println!("{}~{} :  {:?}", sl, sr, self.tree);
    }

    fn count(&mut self) -> i32 {
        self.tree[0] as i32
    }
}

/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */
struct Solution {}

#[cfg(test)]
mod tests {
    use super::CountIntervals;

    #[test]
    fn it_works() {
        let mut obj = CountIntervals::new();
        // obj.add(2, 3);
        // obj.add(7, 10);
        // println!("{}", obj.count());
        // obj.add(5, 8);
        // println!("{}", obj.count());

        obj.add(252, 751);
        println!();

        obj.add(252, 751);
        println!();

        obj.add(259, 751);

        println!("{}", obj.count());
    }
}
