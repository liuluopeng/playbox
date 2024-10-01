struct MyCalendar {
    tree: Vec<Option<bool>>,
    lazy: Vec<usize>,
    size: usize,
    left_seg_addr_list: Vec<Option<usize>>,
    right_seg_addr_list: Vec<Option<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        let my_calendar = MyCalendar {
            tree: vec![Some(false)],
            lazy: vec![0],
            left_seg_addr_list: vec![],
            right_seg_addr_list: vec![],
            size: 1_000_000_001,
        };

        my_calendar
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let start = start as usize;
        let end = end as usize - 1;

        if self.exsit(start, end, 0, self.size, 0) {
            return false;
        }

        // println!("{:?} {:?} 里面是否有日程", start, end);
        // self.debug(0, 0, self.size);

        // for k in start..=end {
        //     self.single_change(k, 0, self.size, 0);
        // }

        self.add(start, end, 0, self.size, 0);
        true
    }

    // true: [start, end]氛围内存在至少一天被订阅.
    fn exsit(&mut self, start: usize, end: usize, l: usize, r: usize, idx: usize) -> bool {
        while idx >= self.left_seg_addr_list.len() {
            self.left_seg_addr_list.push(None);
            self.right_seg_addr_list.push(None);
        }

        // println!("{:?} {:?}  {}", l, r, idx);

        if start <= l && r <= end {
            if let Some(days) = self.tree[idx] {
                return days == true;
            } else {
                return false;
            }
        }

        let mid = (r + l) >> 1;

        if self.left_seg_addr_list[idx].is_none() {
            self.tree.push(None);
            self.lazy.push(0);
            self.left_seg_addr_list[idx] = Some(self.tree.len() - 1);
        }

        if self.right_seg_addr_list[idx].is_none() {
            self.tree.push(None);
            self.lazy.push(0);
            self.right_seg_addr_list[idx] = Some(self.tree.len() - 1);
        }

        if self.lazy[idx] != 0 {
            let l_sub_idx = self.left_seg_addr_list[idx].unwrap();
            self.lazy[l_sub_idx] = mid - l + 1;
            self.tree[l_sub_idx] = Some(true);
            let r_sub_idx = self.right_seg_addr_list[idx].unwrap();
            self.lazy[r_sub_idx] = r - mid;
            self.tree[r_sub_idx] = Some(true);
            self.lazy[idx] = 0;
        }

        let mut res = false;

        if start <= mid {
            if self.left_seg_addr_list[idx].is_none() {
                self.tree.push(None);
                self.lazy.push(0);
                self.left_seg_addr_list[idx] = Some(self.tree.len() - 1);
            }
            res |= self.exsit(start, end, l, mid, self.left_seg_addr_list[idx].unwrap());
        }
        if mid + 1 <= end {
            if self.right_seg_addr_list[idx].is_none() {
                self.tree.push(None);
                self.lazy.push(0);
                self.right_seg_addr_list[idx] = Some(self.tree.len() - 1);
            }

            res |= self.exsit(
                start,
                end,
                mid + 1,
                r,
                self.right_seg_addr_list[idx].unwrap(),
            );
        }

        res
    }

    // 把[start..end]范围全都变成1
    fn add(&mut self, start: usize, end: usize, l: usize, r: usize, idx: usize) {
        while idx >= self.left_seg_addr_list.len() {
            self.left_seg_addr_list.push(None);
            self.right_seg_addr_list.push(None);
        }
        if start <= l && r <= end {
            self.lazy[idx] = r - l + 1;
            self.tree[idx] = Some(true);

            return;
        }

        let mid = (l + r) >> 1;

        if self.left_seg_addr_list[idx].is_none() {
            self.tree.push(None);
            self.lazy.push(0);
            self.left_seg_addr_list[idx] = Some(self.tree.len() - 1);
        }
        if self.right_seg_addr_list[idx].is_none() {
            self.tree.push(None);
            self.lazy.push(0);
            self.right_seg_addr_list[idx] = Some(self.tree.len() - 1);
        }

        let l_sub_idx = self.left_seg_addr_list[idx].unwrap();
        let r_sub_idx = self.right_seg_addr_list[idx].unwrap();
        if self.lazy[idx] != 0 {
            self.lazy[l_sub_idx] = mid - l + 1;
            self.tree[l_sub_idx] = Some(true);

            self.lazy[r_sub_idx] = r - mid;
            self.tree[r_sub_idx] = Some(true);
            self.lazy[idx] = 0;
        }
        if start <= mid {
            self.add(start, end, l, mid, l_sub_idx);
        }
        if mid + 1 <= end {
            self.add(start, end, mid + 1, r, r_sub_idx);
        }

        let mut left_res = false;
        if self.left_seg_addr_list[idx].is_some() {
            match self.tree[self.left_seg_addr_list[idx].unwrap()] {
                None => {}
                Some(v) => {
                    left_res |= v;
                }
            }
        }
        let mut right_res = false;
        if self.right_seg_addr_list[idx].is_some() {
            match self.tree[self.right_seg_addr_list[idx].unwrap()] {
                None => {}
                Some(v) => {
                    right_res |= v;
                }
            }
        }

        self.tree[idx] = Some(left_res | right_res);
    }

    fn debug(&self, idx: usize, left_resp: usize, right_resp: usize) {
        println!(
            "区间{}~{} \t {}  \t 已使用天数:{:?}",
            left_resp, right_resp, idx, self.tree[idx]
        );

        let mid_resp = (left_resp + right_resp) >> 1;
        if idx < self.left_seg_addr_list.len() {
            if let Some(k) = self.left_seg_addr_list[idx] {
                self.debug(k, left_resp, mid_resp);
            }

            if let Some(k) = self.right_seg_addr_list[idx] {
                self.debug(k, mid_resp + 1, right_resp);
            }
        }
    }
}

/**
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

struct Solution;

#[cfg(test)]
mod tests {
    use super::MyCalendar;

    #[test]
    fn it_works() {
        //  * Your MyCalendar object will be instantiated and called as such:
        let mut obj = MyCalendar::new();

        assert_eq!(true, obj.book(23, 32));

        assert_eq!(true, obj.book(42, 50));

        assert_eq!(true, obj.book(6, 14));

        assert_eq!(false, obj.book(0, 7));
    }
}
