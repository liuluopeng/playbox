// struct MyCalendarThree_duishuqi {
//     tree: Vec<i32>,
//     size: usize,
// }

// impl MyCalendarThree_duishuqi {
//     fn new() -> Self {
//         let size = 500;
//         let tree = vec![0; size * 4];

//         Self {
//             tree: tree,
//             size: size,
//         }
//     }

//     fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
//         let start = start_time as usize;
//         let end = end_time as usize - 1;

//         for k in start..=end {
//             // println!("k + 1 {} ------------------------------------", k);
//             self.single_update(k, 0, self.size - 1, 1);
//         }
//         // self.debug(0, self.size - 1, 1);
//         // println!("");
//         let res = self.query(0, self.size - 1, 0, self.size - 1, 1) as i32;

//         res
//     }

//     fn single_update(&mut self, date: usize, l: usize, r: usize, idx: usize) {
//         // println!("l {}  r{}  idx{}", l, r, idx);
//         if l == r {
//             // println!(
//             //     "l: r: {} {}  idx:{}  self.tree:{}",
//             //     l, r, idx, self.tree[idx]
//             // );
//             self.tree[idx] = self.tree[idx] + 1;
//             return;
//         }

//         let mid = (l + r) / 2;
//         if date <= mid {
//             self.single_update(date, l, mid, idx * 2);
//         } else {
//             self.single_update(date, mid + 1, r, idx * 2 + 1);
//         }

//         let left_res = self.tree[idx * 2];
//         let right_res = self.tree[idx * 2 + 1];
//         self.tree[idx] = left_res.max(right_res);
//     }

//     fn query(&mut self, start: usize, end: usize, l: usize, r: usize, idx: usize) -> i32 {
//         if start <= l && r <= end {
//             return self.tree[idx];
//         }

//         let mut max_cover = 0;
//         let mid = (l + r) / 2;
//         if start <= mid {
//             max_cover = max_cover.max(self.query(start, end, l, mid, idx * 2));
//         }

//         if mid + 1 <= end {
//             max_cover = max_cover.max(self.query(start, end, mid + 1, r, idx * 2 + 1));
//         }

//         max_cover
//     }

//     pub fn debug(&self, sl: usize, sr: usize, idx: usize) {
//         if sl == sr {
//             println!("范围:{}-{} idx:{} 数据 {:?}", sl, sr, idx, self.tree[idx]);
//             return;
//         }

//         // 0 ~ size - 1
//         let mid = (sl + sr) >> 1;

//         println!("范围:{}-{} idx:{} 数据 {:?}", sl, sr, idx, self.tree[idx]);

//         Self::debug(&self, sl, mid, idx << 1);
//         Self::debug(&self, mid + 1, sr, idx << 1 | 1);
//     }
// }

struct MyCalendarThree {
    tree: Vec<usize>, // 重复的次数
    lazy: Vec<usize>,
    size: usize,
    left_seg_addr_list: Vec<Option<usize>>,
    right_seg_addr_list: Vec<Option<usize>>,
}

impl MyCalendarThree {
    fn new() -> Self {
        let my_calendar: MyCalendarThree = MyCalendarThree {
            tree: vec![0],
            lazy: vec![0],
            left_seg_addr_list: vec![None],
            right_seg_addr_list: vec![None],
            size: 1_000_000_001,
            // size: 100,
        };

        my_calendar
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        let start = start as usize;
        let end = end as usize - 1;

        self.add(start, end, 0, self.size, 0);

        self.max_cover(0, self.size, 0, self.size, 0) as i32
    }

    // 不够的时候扩充
    fn extend(&mut self, father_idx: usize, child_side_l: bool) {
        // self.tree.push(None);
        // self.tree.push(Some(0));

        self.tree.push(0);

        self.lazy.push(0);
        self.left_seg_addr_list.push(None);
        self.right_seg_addr_list.push(None);

        let need_idx = self.tree.len() - 1;

        if child_side_l {
            self.left_seg_addr_list[father_idx] = Some(need_idx);
        } else {
            self.right_seg_addr_list[father_idx] = Some(need_idx);
        }
    }

    fn pushdown(&mut self, idx: usize, l: usize, r: usize) {
        if self.left_seg_addr_list[idx].is_none() {
            self.extend(idx, true);
        }

        if self.right_seg_addr_list[idx].is_none() {
            self.extend(idx, false);
        }

        let l_sub_idx = self.left_seg_addr_list[idx].unwrap();
        let r_sub_idx = self.right_seg_addr_list[idx].unwrap();

        self.lazy[l_sub_idx] += self.lazy[idx];
        self.tree[l_sub_idx] += self.lazy[idx];
        self.lazy[r_sub_idx] += self.lazy[idx];
        self.tree[r_sub_idx] += self.lazy[idx];
        self.lazy[idx] = 0;
    }

    fn pushup(&mut self, idx: usize) {
        let left_res = self.tree[self.left_seg_addr_list[idx].unwrap()];

        let mut right_res = self.tree[self.right_seg_addr_list[idx].unwrap()];
        self.tree[idx] = left_res.max(right_res);
    }

    fn max_cover(&mut self, start: usize, end: usize, l: usize, r: usize, idx: usize) -> usize {
        // 0 没有满
        // 1 没有满

        if start <= l && r <= end {
            return self.tree[idx];
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
            self.lazy[idx] += 1;

            self.tree[idx] += 1;
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

#[cfg(test)]
mod tests {
    use super::MyCalendarThree;

    #[test]
    fn it_works() {
        let mut cal = MyCalendarThree::new();

        assert_eq!(1, cal.book(10, 20));
        assert_eq!(1, cal.book(50, 60));
        assert_eq!(2, cal.book(10, 40));
        assert_eq!(3, cal.book(5, 15));
        assert_eq!(3, cal.book(5, 10));
        assert_eq!(3, cal.book(25, 55));

        let mut cal = MyCalendarThree::new();

        let dd = [
            [47, 50],
            [1, 10],
            [27, 36],
            [40, 47],
            [20, 27],
            [15, 23],
            [10, 18],
            [27, 36],
            [17, 25],
            [8, 17],
            [24, 33],
            [23, 28],
            [21, 27],
            [47, 50],
            [14, 21],
            [26, 32],
            [16, 21],
            [2, 7],
            [24, 33],
            [6, 13],
            [44, 50],
            [33, 39],
            [30, 36],
            [6, 15],
            [21, 27],
            [49, 50],
            [38, 45],
            [4, 12],
            [46, 50],
            [13, 21],
        ];

        let res = [
            1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7,
            7,
        ];

        for i in 0..res.len() {
            assert_eq!(res[i], cal.book(dd[i][0], dd[i][1]))
        }
    }
}
