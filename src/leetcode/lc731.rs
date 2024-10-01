/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

struct MyCalendarTwo {
    tree: Vec<Option<usize>>, // 重复的次数
    lazy: Vec<usize>,
    size: usize,
    left_seg_addr_list: Vec<Option<usize>>,
    right_seg_addr_list: Vec<Option<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        let my_calendar = MyCalendarTwo {
            tree: vec![Some(0)],
            lazy: vec![0],
            left_seg_addr_list: vec![None],
            right_seg_addr_list: vec![None],
            size: 1_000_000_001,
            // size: 100,
        };

        my_calendar
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let start = start as usize;
        let end = end as usize - 1;

        if self.max_cover(start, end, 0, self.size, 0) >= 2 {
            return false;
        }

        // println!("{:?} {:?} ", start, end,);
        // self.debug(0, 0, self.size);
        // println!(
        //     "{:?} {:?} *********************************************查询结束",
        //     start, end,
        // );

        self.add(start, end, 0, self.size, 0);

        // self.debug(0, 0, self.size);
        // println!(
        //     "{:?} {:?} *********************************************更新结束",
        //     start, end,
        // );

        true
    }

    fn extend(&mut self, father_idx: usize, child_side_l: bool) {
        self.tree.push(None);
        self.lazy.push(0);
        self.left_seg_addr_list.push(None);
        self.right_seg_addr_list.push(None);

        let need_idx = self.tree.len() - 1;

        if child_side_l {
            self.left_seg_addr_list[father_idx] = Some(need_idx);
        } else {
            self.right_seg_addr_list[father_idx] = Some(need_idx);
        }

        // println!(
        //     "new node idx {:?}  {:?} left addr list {:?} right addr list {:?}",
        //     need_idx, self.tree, self.left_seg_addr_list, self.right_seg_addr_list
        // );

        // while need_idx >= self.tree.len() {
        //     if child_side_l {
        //         self.left_seg_addr_list[father_idx] = Some(father_idx);
        //     } else {
        //         self.right_seg_addr_list[father_idx] = Some(father_idx);
        //     }

        //     self.tree.push(None);
        //     self.lazy.push(0);
        //     self.left_seg_addr_list.push(None);
        //     self.right_seg_addr_list.push(None);
        // }
    }

    fn pushdown(&mut self, idx: usize, l: usize, r: usize) {
        let mid = (l + r) >> 1;

        if self.left_seg_addr_list[idx].is_none() {
            self.extend(idx, true);
        }

        if self.right_seg_addr_list[idx].is_none() {
            self.extend(idx, false);
        }

        let l_sub_idx = self.left_seg_addr_list[idx].unwrap();
        let r_sub_idx = self.right_seg_addr_list[idx].unwrap();

        if self.lazy[idx] != 0 {
            self.lazy[l_sub_idx] = mid - l + 1;

            if let Some(v) = self.tree[l_sub_idx] {
                self.tree[l_sub_idx] = Some(v + 1);
            } else {
                // self.tree[l_sub_idx] = Some(0);

                // if self.tree[idx].unwrap() == 0 {
                //     self.tree[l_sub_idx] = Some(0);
                // }

                self.tree[l_sub_idx] = Some(self.tree[idx].unwrap());
            }

            if let Some(v) = self.tree[r_sub_idx] {
                self.tree[r_sub_idx] = Some(v + 1);
            } else {
                // self.tree[r_sub_idx] = Some(0);

                // if self.tree[idx].unwrap() == 0 {
                //     self.tree[r_sub_idx] = Some(0);
                // }

                self.tree[r_sub_idx] = Some(self.tree[idx].unwrap());
            }
            self.lazy[r_sub_idx] = r - mid;

            self.lazy[idx] = 0;
        }

        if self.tree[l_sub_idx].is_none() {
            // if self.tree[idx].unwrap() == 0 {
            //     self.tree[l_sub_idx] = Some(0);
            // }

            self.tree[l_sub_idx] = Some(self.tree[idx].unwrap());
        }
        if self.tree[r_sub_idx].is_none() {
            // if self.tree[idx].unwrap() == 0 {
            //     self.tree[r_sub_idx] = Some(0);
            // }

            self.tree[r_sub_idx] = Some(self.tree[idx].unwrap());
        }
    }

    fn pushup(&mut self, idx: usize) {
        // println!("push up");

        let mut left_res = 0;

        match self.tree[self.left_seg_addr_list[idx].unwrap()] {
            None => {
                // self.tree[self.left_seg_addr_list[idx].unwrap()] = Some(0);

                // println!("idx left NONE {}", idx);
            }
            Some(v) => {
                left_res = left_res.max(v);
            }
        }

        let mut right_res = 0;

        match self.tree[self.right_seg_addr_list[idx].unwrap()] {
            None => {
                // self.tree[self.right_seg_addr_list[idx].unwrap()] = Some(0);

                // println!("idx right NONE {}", idx);
            }
            Some(v) => {
                right_res = right_res.max(v);
            }
        }

        self.tree[idx] = Some(left_res.max(right_res))
    }

    fn max_cover(&mut self, start: usize, end: usize, l: usize, r: usize, idx: usize) -> usize {
        // 0 没有满
        // 1 没有满

        if start <= l && r <= end {
            if let Some(cover) = self.tree[idx] {
                // cover:记录到的最高重复次数
                return cover;
            } else {
                self.debug(0, 0, self.size);
                panic!("{}  {}~{}Found None", idx, l, r);
                // return 0;
            }
        }

        self.pushdown(idx, l, r);

        let mid = (r + l) >> 1;

        let mut left_res = 0;
        let mut right_res = 0;
        if start <= mid {
            left_res = self.max_cover(start, end, l, mid, self.left_seg_addr_list[idx].unwrap());
        }
        if mid + 1 <= end {
            right_res = self.max_cover(
                start,
                end,
                mid + 1,
                r,
                self.right_seg_addr_list[idx].unwrap(),
            );
        }

        let res = left_res.max(right_res);

        // println!(
        //     "{:?} {:?} {:?}   max res:  {} left:{} right:{}",
        //     l, r, idx, res, left_res, right_res
        // );

        res
    }

    fn add(&mut self, start: usize, end: usize, l: usize, r: usize, idx: usize) {
        if start <= l && r <= end {
            self.lazy[idx] = r - l + 1;

            match self.tree[idx] {
                None => self.tree[idx] = Some(1),
                Some(v) => self.tree[idx] = Some(v + 1),
            }

            return;
        }

        self.pushdown(idx, l, r);

        let mid = (l + r) >> 1;

        if start <= mid {
            if self.left_seg_addr_list[idx].is_none() {
                self.extend(idx, true);
            }

            self.add(start, end, l, mid, self.left_seg_addr_list[idx].unwrap());
        }
        if mid + 1 <= end {
            if self.right_seg_addr_list[idx].is_none() {
                self.extend(idx, false);
            }

            self.add(
                start,
                end,
                mid + 1,
                r,
                self.right_seg_addr_list[idx].unwrap(),
            );
        }

        self.pushup(idx);
    }

    fn debug(&self, idx: usize, left_resp: usize, right_resp: usize) {
        println!(
            "区间{}~{} \t idx:{}  \t  最多一天被预定的次数:{:?}",
            left_resp, right_resp, idx, self.tree[idx]
        );

        let mid_resp = (left_resp + right_resp) >> 1;

        if left_resp == mid_resp {
            return;
        }

        if let Some(k) = self.left_seg_addr_list[idx] {
            self.debug(k, left_resp, mid_resp);
        }

        if let Some(k) = self.right_seg_addr_list[idx] {
            self.debug(k, mid_resp + 1, right_resp);
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::MyCalendarTwo;

    #[test]
    fn it_works() {
        let mut cal = MyCalendarTwo::new();
        assert_eq!(true, cal.book(0, 51));
        assert_eq!(true, cal.book(0, 26));
        // assert_eq!(false, cal.book(10, 21));

        let mut cal = MyCalendarTwo::new();
        assert_eq!(true, cal.book(10, 20));
        assert_eq!(true, cal.book(50, 60));
        assert_eq!(true, cal.book(10, 40));
        assert_eq!(false, cal.book(5, 15));
        assert_eq!(true, cal.book(5, 10));
        assert_eq!(true, cal.book(25, 55));

        let mut cal = MyCalendarTwo::new();
        assert_eq!(true, cal.book(24, 40));
        assert_eq!(true, cal.book(43, 50));
        assert_eq!(true, cal.book(27, 43));
        assert_eq!(true, cal.book(5, 21));
        assert_eq!(false, cal.book(30, 40));
        assert_eq!(false, cal.book(14, 29));
    }
}
