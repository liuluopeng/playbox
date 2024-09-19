use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    // main start: 这部分测试成功之后粘贴到mian函数里面.
    let mut lines = reader.lines();
    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let opr_total = parts[1];

    let ori_data: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut opr_list: Vec<Vec<i32>> = vec![];

    for _ in 0..opr_total {
        opr_list.push(
            lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    }

    // main end. 从下面开始是 和 题目的输入 无关的 .
    let mut buf = vec![];
    slove(&mut buf, ori_data, opr_list);
    // 处理输入
    let output = String::from_utf8(buf).expect("无法解析为 UTF-8 字符串");
    println!("{}", output);
}

pub fn slove(result_buffer: &mut Vec<u8>, ori_data: Vec<i32>, opr_list: Vec<Vec<i32>>) {
    let mut buf_writer = BufWriter::new(result_buffer);

    let mut seg_tree = SegTree::new(&ori_data);

    for op in opr_list {
        /*

        */

        match op[0] {
            0 => {
                //         - `0 l r` 把 $[l, r]$ 区间内的所有数全变成 $0$；
                let start = op[1] as usize;
                let end = op[2] as usize;

                // for k in start..=end {
                //     seg_tree.single_set(false, true, false, k, 0, seg_tree.size - 1, 1);
                // }

                seg_tree.reset_section(0, start, end, 0, seg_tree.size - 1, 1);
            }
            1 => {
                //         - `1 l r` 把 $[l, r]$ 区间内的所有数全变成 $1$；
                let start = op[1] as usize;
                let end = op[2] as usize;

                // for k in start..=end {
                //     seg_tree.single_set(true, false, false, k, 0, seg_tree.size - 1, 1);
                // }

                seg_tree.reset_section(1, start, end, 0, seg_tree.size - 1, 1);
            }
            2 => {
                //         - `2 l r` 把 $[l,r]$ 区间内的所有数全部取反，也就是说把所有的 $0$ 变成 $1$，把所有的 $1$ 变成 $0$；

                let start = op[1] as usize;
                let end = op[2] as usize;

                // for k in start..=end {
                //     seg_tree.single_set(false, false, true, k, 0, seg_tree.size - 1, 1);
                // }

                seg_tree.reverse_section(start, end, 0, seg_tree.size - 1, 1);
            }
            3 => {
                //         - `3 l r` 询问 $[l, r]$ 区间内总共有多少个 $1$；
                let query_res =
                    seg_tree.get_1_counter(op[1] as usize, op[2] as usize, 0, seg_tree.size - 1, 1);
                writeln!(buf_writer, "{}", query_res).unwrap();
            }
            4 => {
                //         - `4 l r` 询问 $[l, r]$ 区间内最多有多少个连续的 $1$。
                let query_res =
                    seg_tree.get_longest_1(op[1] as usize, op[2] as usize, 0, seg_tree.size - 1, 1);
                writeln!(buf_writer, "{}", query_res.0).unwrap();
            }

            _ => {
                panic!();
            }
        }

        // println!("OP: {:?}\n {:?} \n\n", op, seg_tree);
    }
}

#[derive(Debug)]
struct SegTree {
    // 0:longest1  1:pre1  2:suf1       3:longest0   4:pre0  5:suf0
    tree: Vec<(usize, usize, usize, usize, usize, usize, usize)>,
    ori_data: Vec<i32>,
    size: usize, // 区间的大小
    reset_lazy: Vec<Option<usize>>,
    reverse_lazy: Vec<bool>, // 当前区间有无反转 积累 的反转任务
}

impl SegTree {
    pub fn debug(&self, sl: usize, sr: usize, idx: usize) {
        if sl == sr {
            println!("范围:{}-{} idx:{} 数据 {:?}", sl, sr, idx, self.tree[idx]);
            return;
        }

        // 0 ~ size - 1
        let mid = (sl + sr) / 2;

        println!("范围:{}-{} idx:{} 数据 {:?}", sl, sr, idx, self.tree[idx]);

        Self::debug(&self, sl, mid, idx * 2);
        Self::debug(&self, mid + 1, sr, idx * 2 + 1);
    }

    pub fn new(data: &Vec<i32>) -> Self {
        let tree = vec![(0, 0, 0, 0, 0, 0, 0); data.len() * 4];
        let reset_lazy = vec![None; data.len() * 4];
        let reverse_lazy = vec![false; data.len() * 4];
        let mut seg_tree = SegTree {
            tree,
            ori_data: data.clone(),
            size: data.len(),
            reset_lazy,
            reverse_lazy,
        };

        seg_tree.build(0, seg_tree.size - 1, 1);
        seg_tree
    }

    fn summarizing(&mut self, sl: usize, sr: usize, idx: usize) {
        let mid = (sl + sr) / 2;

        let left_res = self.tree[idx * 2];
        let right_res = self.tree[idx * 2 + 1];

        // 新的最长的连续1的长度:  左侧最长的连续1  右侧最长连续1长度  还有 [...11111][1111...]这样拼接的可能性
        let t0_longest1 = left_res.0.max(right_res.0).max(left_res.2 + right_res.1);

        // 新的前缀1的长度:   需要看看有0中断没有.
        let t1_pre1 = match (mid - sl + 1) == left_res.1 {
            true => left_res.1 + right_res.1, // [11111][111...]
            false => left_res.1,              // [11111000][11...]
        };

        // 新的后缀1的长度:
        let t2_suf1 = match (sr - (mid + 1) + 1) == right_res.2 {
            true => left_res.2 + right_res.2,
            false => right_res.2,
        };

        let t3_longest0 = left_res.3.max(right_res.3).max(left_res.5 + right_res.4);

        let t4_pre0 = match (mid - sl + 1) == left_res.4 {
            true => (mid - sl + 1) + right_res.4,
            false => left_res.4,
        };

        let t5_suf0 = match (sr - mid) == right_res.5 {
            // true => left_res.4 + right_res.5,
            true => left_res.5 + (sr - mid),
            false => right_res.5,
        };

        let t6_counter_1 = left_res.6 + right_res.6;

        self.tree[idx] = (
            t0_longest1,
            t1_pre1,
            t2_suf1,
            t3_longest0,
            t4_pre0,
            t5_suf0,
            t6_counter_1,
        );
    }

    fn build(
        &mut self,

        // tree: &mut Vec<(usize, usize, usize, usize, usize, usize)>,
        l: usize,
        r: usize,
        idx: usize,
    ) {
        if l == r {
            match self.ori_data[l] {
                0 => {
                    self.tree[idx] = (0, 0, 0, 1, 1, 1, 0);
                }
                1 => {
                    self.tree[idx] = (1, 1, 1, 0, 0, 0, 1);
                }
                _ => {
                    panic!();
                }
            }
            return;
        }

        let mid = (l + r) / 2;

        Self::build(self, l, mid, idx * 2);
        Self::build(self, mid + 1, r, idx * 2 + 1);

        Self::summarizing(self, l, r, idx);
    }

    pub fn reset_section(
        &mut self,
        rest: usize, // 指定的数字
        jobl: usize,
        jobr: usize,
        sl: usize,
        sr: usize,
        idx: usize,
    ) {
        if jobl <= sl && sr <= jobr {
            Self::lazy_update(self, idx, sr - sl + 1, rest);
            return;
        }

        let mid = (sl + sr) / 2;
        Self::down(self, idx, mid - sl + 1, sr - mid);

        if jobl <= mid {
            Self::reset_section(self, rest, jobl, jobr, sl, mid, idx * 2);
        }

        if mid + 1 <= jobr {
            Self::reset_section(self, rest, jobl, jobr, mid + 1, sr, idx * 2 + 1);
        }

        Self::summarizing(self, sl, sr, idx);
    }

    fn down(&mut self, idx: usize, ln: usize, rn: usize) {
        if self.reset_lazy[idx].is_some() {
            let rest = self.reset_lazy[idx].unwrap();

            Self::lazy_update(self, idx * 2, ln, rest);
            Self::lazy_update(self, idx * 2 + 1, rn, rest);

            self.reset_lazy[idx] = None;
        }

        if self.reverse_lazy[idx] {
            Self::lazy_reverse(self, idx * 2, ln);
            Self::lazy_reverse(self, idx * 2 + 1, rn);

            self.reverse_lazy[idx] = false;
        }
    }

    fn lazy_update(&mut self, idx: usize, n: usize, target_rest: usize) {
        match target_rest {
            // 暂存任务的同时,利用规则修改tree.
            1 => {
                self.reset_lazy[idx] = Some(1);

                self.tree[idx] = (n, n, n, 0, 0, 0, n);

                self.reverse_lazy[idx] = false; // 优先级
            }
            0 => {
                self.reset_lazy[idx] = Some(0);
                self.tree[idx] = (0, 0, 0, n, n, n, 0);

                self.reverse_lazy[idx] = false; // 优先级
            }
            _ => {
                panic!()
            }
        }
    }

    fn lazy_reverse(&mut self, idx: usize, n: usize) {
        let (succ1, pre1, suf1, succ0, pre0, suf0, sum1) = self.tree[idx];

        self.tree[idx] = (succ0, pre0, suf0, succ1, pre1, suf1, n - sum1);

        self.reverse_lazy[idx] = !self.reverse_lazy[idx];
    }

    // 逐个修改
    pub fn single_set(
        &mut self,
        set_1: bool,
        set_0: bool,
        set_negation: bool,
        target_idx: usize,
        sl: usize,
        sr: usize,
        idx: usize,
    ) {
        if sl == sr {
            if set_1 {
                self.tree[idx] = (1, 1, 1, 0, 0, 0, 1);
            } else if set_0 {
                self.tree[idx] = (0, 0, 0, 1, 1, 1, 0);
            } else if set_negation {
                // 看看原来是什么
                if self.tree[idx].6 == 1 {
                    self.tree[idx] = (0, 0, 0, 1, 1, 1, 0);
                } else {
                    self.tree[idx] = (1, 1, 1, 0, 0, 0, 1);
                }
            }

            return;
        }

        let mid = (sl + sr) / 2;

        // 处理两侧
        if target_idx <= mid {
            Self::single_set(
                self,
                set_1,
                set_0,
                set_negation,
                target_idx,
                sl,
                mid,
                idx * 2,
            );
        } else {
            Self::single_set(
                self,
                set_1,
                set_0,
                set_negation,
                target_idx,
                mid + 1,
                sr,
                idx * 2 + 1,
            );
        }

        Self::summarizing(self, sl, sr, idx);
    }

    pub fn reverse_section(&mut self, jobl: usize, jobr: usize, sl: usize, sr: usize, idx: usize) {
        if jobl <= sl && sr <= jobr {
            Self::lazy_reverse(self, idx, sr - sl + 1);
            return;
        }

        let mid = (sr + sl) / 2;
        Self::down(self, idx, mid - sl + 1, sr - mid);

        if jobl <= mid {
            Self::reverse_section(self, jobl, jobr, sl, mid, idx * 2);
        }
        if mid + 1 <= jobr {
            Self::reverse_section(self, jobl, jobr, mid + 1, sr, idx * 2 + 1);
        }

        Self::summarizing(self, sl, sr, idx);
    }

    pub fn get_1_counter(
        &mut self,
        jobl: usize,
        jobr: usize,
        sl: usize,
        sr: usize,
        idx: usize,
    ) -> usize {
        if jobl <= sl && sr <= jobr {
            return self.tree[idx].6;
        }

        let mut res = 0;

        let mid = (sl + sr) / 2;
        Self::down(self, idx, mid - sl + 1, sr - mid);

        if jobl <= mid {
            res += Self::get_1_counter(self, jobl, jobr, sl, mid, idx * 2);
        }
        if mid + 1 <= jobr {
            res += Self::get_1_counter(self, jobl, jobr, mid + 1, sr, idx * 2 + 1);
        }
        res
    }

    //
    pub fn get_longest_1(
        &mut self,
        jobl: usize,
        jobr: usize,
        sl: usize,
        sr: usize,
        idx: usize,
    ) -> (usize, usize, usize) {
        if jobl <= sl && sr <= jobr {
            // [l   sl...sr      r]
            let v = self.tree[idx];

            return (v.0, v.1, v.2);
        }

        let mid = (sl + sr) / 2;
        Self::down(self, idx, mid - sl + 1, sr - mid);

        if jobr <= mid {
            return Self::get_longest_1(self, jobl, jobr, sl, mid, idx * 2);
        }
        if jobl > mid {
            return Self::get_longest_1(self, jobl, jobr, mid + 1, sr, idx * 2 + 1);
        }

        let left_res = Self::get_longest_1(self, jobl, jobr, sl, mid, idx * 2);
        let right_res = Self::get_longest_1(self, jobl, jobr, mid + 1, sr, idx * 2 + 1);

        let long1 = (left_res.2 + right_res.1).max(left_res.0).max(right_res.0);

        let mut pre = 0;

        match left_res.0 < mid - jobl.max(sl) + 1 {
            true => {
                pre = left_res.1;
            }
            false => {
                pre = left_res.1 + right_res.1;
            }
        }

        let mut suf = 0;
        match right_res.0 < jobr.min(sr) - mid {
            true => {
                suf = right_res.2;
            }
            false => {
                suf = left_res.2 + right_res.2;
            }
        }

        (long1, pre, suf)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Cursor},
    };

    use rand::Rng;

    use super::{main, slove, SegTree};

    #[test]
    /// 直接执行, 手动粘贴.
    fn test_main() {
        main();
    }

    #[test]
    fn test_1() {
        let answer_input_path = "testcase/P2572_1.in";
        let answer_output_path = "testcase/P2572_1.out";

        let file: File = File::open(answer_input_path).expect("无法打开文件");
        let reader: BufReader<File> = BufReader::new(file);

        // main start: 这部分测试成功之后粘贴到mian函数里面.
        let mut lines = reader.lines();

        // println!("{:?}", lines);

        // 读取第一行
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = first_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let opr_total = parts[1];

        let ori_data: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut opr_list: Vec<Vec<i32>> = vec![];

        for _ in 0..opr_total {
            opr_list.push(
                lines
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        }

        // main end. 从下面开始是 和 题目的输入 无关的 .
        let mut buf = vec![];
        slove(&mut buf, ori_data, opr_list);

        // 将缓冲区转化为字符串，方便逐行比较
        let buffer_cursor = Cursor::new(buf);
        let buffer_reader = BufReader::new(buffer_cursor);

        // Step 4: 读取 output.txt 作为正确答案
        let output_file = File::open(answer_output_path).unwrap();
        let output_reader = BufReader::new(output_file);

        // Step 5: 比较每行结果与正确答案逐行对比
        let mut is_correct = true;

        let mut line_counter = 1;
        for (output_line, result_line) in output_reader.lines().zip(buffer_reader.lines()) {
            let output_line = output_line.unwrap();
            let result_line = result_line.unwrap();

            if output_line.trim() != result_line.trim() {
                println!(
                    "行号: {} 错误：预期输出 '{}' 与结果 '{}' 不匹配",
                    line_counter, output_line, result_line
                );
                is_correct = false;
            }

            assert_eq!(output_line.trim(), result_line.trim());
            line_counter += 1;
        }

        if is_correct {
            println!("验证通过，所有结果均正确！");
        } else {
            println!("验证失败，有不匹配的结果。");
        }
        println!("测试结束");
    }

    #[test]
    fn duishuqi() {
        let mut brute_ = vec![];

        let mut rng = rand::thread_rng();

        let size = 1000;
        // 生成数据
        for i in 0..size {
            brute_.push(rng.gen_range(0..2));
        }

        // let mut brute_ = vec![0, 1, 0, 1, 1, 1, 0, 0, 0, 0];
        let brute_copy = brute_.clone();

        // 生成随机操作
        let mut op_list = vec![];
        for i in 0..size {
            let op = rng.gen_range(0..5);

            let start = rng.gen_range(0..size);
            let end = rng.gen_range(start..size);

            op_list.push(vec![op, start, end]);
        }

        // let mut op_list = [
        //     // [0, 1, 3],
        //     // [4, 1, 7],
        //     // [3, 6, 7],
        //     // [4, 0, 1],
        //     [2, 3, 9],
        //     // [3, 6, 7],
        //     [4, 4, 9],
        //     // [0, 1, 3],
        //     // [0, 6, 7],
        //     // [4, 0, 7],
        // ];

        // main end. 从下面开始是 和 题目的输入 无关的 .
        // let mut buf = vec![];

        let mut seg_tree = SegTree::new(&brute_copy);

        println!("ori data: {:?} op: {:?}", brute_, op_list);

        for op in op_list {
            match op[0] {
                0 => {
                    //         - `0 l r` 把 $[l, r]$ 区间内的所有数全变成 $0$；
                    let start = op[1] as usize;
                    let end = op[2] as usize;

                    println!("reset 0  {}-{}", start, end);

                    seg_tree.reset_section(0, start, end, 0, seg_tree.size - 1, 1);

                    for k in start..=end {
                        brute_[k] = 0;
                    }
                }
                1 => {
                    //         - `1 l r` 把 $[l, r]$ 区间内的所有数全变成 $1$；
                    let start = op[1] as usize;
                    let end = op[2] as usize;

                    println!("reset 1  {}-{}", start, end);
                    seg_tree.reset_section(1, start, end, 0, seg_tree.size - 1, 1);

                    for k in start..=end {
                        brute_[k] = 1;
                    }
                }
                2 => {
                    //         - `2 l r` 把 $[l,r]$ 区间内的所有数全部取反，也就是说把所有的 $0$ 变成 $1$，把所有的 $1$ 变成 $0$；

                    let start = op[1] as usize;
                    let end = op[2] as usize;

                    println!("reverse  {}-{}", start, end);
                    seg_tree.reverse_section(start, end, 0, seg_tree.size - 1, 1);

                    for k in start..=end {
                        if brute_[k] == 0 {
                            brute_[k] = 1;
                        } else {
                            brute_[k] = 0;
                        }
                    }
                }
                3 => {
                    //         - `3 l r` 询问 $[l, r]$ 区间内总共有多少个 $1$；

                    let start = op[1] as usize;
                    let end = op[2] as usize;

                    let query_res = seg_tree.get_1_counter(
                        op[1] as usize,
                        op[2] as usize,
                        0,
                        seg_tree.size - 1,
                        1,
                    );

                    println!("all 1   {}-{}  ", start, end);
                    let mut counter = 0;

                    for k in start..=end {
                        if brute_[k] == 1 {
                            counter += 1;
                        }
                    }

                    assert_eq!(query_res, counter);
                }
                4 => {
                    //         - `4 l r` 询问 $[l, r]$ 区间内最多有多少个连续的 $1$。
                    let query_res = seg_tree.get_longest_1(
                        op[1] as usize,
                        op[2] as usize,
                        0,
                        seg_tree.size - 1,
                        1,
                    );

                    let mut curr_ma = 0;

                    let mut curr_idx = op[1] as usize;
                    while curr_idx <= op[2] as usize {
                        let mut tt = 0;

                        if brute_[curr_idx] == 0 {
                            curr_idx += 1;
                            continue;
                        }

                        while curr_idx <= op[2] && brute_[curr_idx] == 1 {
                            curr_idx += 1;
                            tt += 1;
                        }
                        if tt > curr_ma {
                            curr_ma = tt;
                        }
                    }

                    println!("success 1 {}  {}", op[1], op[2]);
                    assert_eq!(curr_ma, query_res.0);
                }

                _ => {
                    panic!();
                }
            }
        }
    }

    #[test]
    fn test_success_1() {
        let ori_data = vec![0, 0, 0, 1, 0, 0, 1, 1, 1, 1];
        let mut seg_tree = SegTree::new(&ori_data);

        assert_eq!(4, seg_tree.get_longest_1(4, 9, 0, seg_tree.size - 1, 1).0);
    }

    #[test]
    fn test_resverse() {
        let ori_data = vec![0, 1, 0, 1, 1, 1, 0, 0, 0, 0];

        // let ori_data = vec![0, 1, 0, 0, 0, 0, 1, 1, 1, 1];

        let mut seg_tree = SegTree::new(&ori_data);
        println!("反转之前:  {:?}", seg_tree);

        seg_tree.debug(0, seg_tree.size - 1, 1);

        seg_tree.reverse_section(3, 9, 0, seg_tree.size - 1, 1);

        println!("反转之后:  {:?}", seg_tree);

        // for i in 0..10 {
        //     println!(
        //         "{:?} {:?}",
        //         seg_tree.get_1_counter(i, i, 0, seg_tree.size - 1, 1),
        //         seg_tree.get_longest_1(i, i, 0, seg_tree.size - 1, 1)
        //     );
        // }

        println!(
            "{:?}",
            seg_tree.get_longest_1(3, 9, 0, seg_tree.size - 1, 1)
        );

        println!("{:?}", seg_tree);

        // for i in 0..10 {
        //     println!(
        //         "{:?}",
        //         seg_tree.get_longest_1(i, 9, 0, seg_tree.size - 1, 1)
        //     );
        // }
    }
}

/*
10 2
0 1 0 1 1 1 0 0 0 0
2 3 9
4 3 9



10 1
0 1 0 1 1 1 0 0 0 0
2 3 9

*/
