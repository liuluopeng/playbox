use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let total_opr = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>()[1];

    let second_line = lines.next().unwrap().unwrap();
    let data = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut op_list = vec![];
    for op in 0..total_opr {
        let line = lines.next().unwrap().unwrap();
        let parts = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        op_list.push(parts);
    }

    let mut res_writer = slove(data, op_list);
    res_writer.flush().unwrap();
    // println!("{:?}", res_writer);
}

pub fn slove(data: Vec<i64>, op_list: Vec<Vec<i64>>) -> BufWriter<Stdout> {
    // 创建一个向标准输出写入的 BufWriter
    let mut buffer = Vec::new();
    let mut writer = BufWriter::new(&mut buffer);

    let mut writer = BufWriter::new(stdout());

    // 使用 writeln! 宏向 BufWriter 写入数据

    let mut set_tree = SegTree::new(data);

    for op in op_list {
        match op[0] {
            1 => {
                set_tree.add_on_range(
                    op[1] as usize - 1,
                    op[2] as usize - 1,
                    op[3],
                    0,
                    set_tree.size - 1,
                    1,
                );

                // let start_idx = op[1] as usize - 1;
                // let end_idx = op[2] as usize - 1;
                // for i in start_idx..=end_idx {
                //     set_tree.sigle_asd(op[3], i, 0, set_tree.size - 1, 1);
                // }
            }
            2 => {
                // println!(
                //     "{}",
                //     set_tree.sum_of_range(
                //         op[1] as usize - 1,
                //         op[2] as usize - 1,
                //         0,
                //         set_tree.size - 1,
                //         1
                //     )
                // );

                // writeln!(res_final, "s");
                writeln!(
                    writer,
                    "{}",
                    set_tree.sum_of_range(
                        op[1] as usize - 1,
                        op[2] as usize - 1,
                        0,
                        set_tree.size - 1,
                        1
                    )
                )
                .unwrap();
            }
            _ => {}
        }
    }

    // 刷新 BufWriter，将缓冲区的数据写入到标准输出
    // writer.flush().unwrap();
    return writer;
}

struct SegTree {
    tree: Vec<i64>,

    size: usize,

    lazy_adder: Vec<i64>,
    // lazy_summer: Vec<i32>,
}
impl SegTree {
    pub fn new(data: Vec<i64>) -> Self {
        let mut tree = vec![0; 4 * data.len()];
        let lazy_adder = vec![0; 4 * data.len()];
        // let mut lazy_summer = vec![0; 4 * data.len()];

        Self::init(&mut tree, 0, data.len() - 1, 1, &data);

        // println!("tree init : {:?}", tree);

        Self {
            tree: tree,
            size: data.len(),
            lazy_adder,
            // lazy_summer,
        }
    }

    // (l, r, idx_tree) 是固定的, 前两个决定第三个参数
    fn init(tree: &mut Vec<i64>, l: usize, r: usize, idx_tree: usize, ori_data: &Vec<i64>) {
        if l == r {
            tree[idx_tree] = ori_data[l];
            return;
        }

        let mid = (l + r) / 2;
        Self::init(tree, l, mid, idx_tree * 2, ori_data);
        Self::init(tree, mid + 1, r, idx_tree * 2 + 1, ori_data);

        tree[idx_tree] = tree[idx_tree * 2] + tree[idx_tree * 2 + 1];
    }

    // lazy[idx]有一个数字, 现在: 交给孩子.
    fn down(&mut self, l_now: usize, r_now: usize, idx_now: usize) {
        // if l_now == r_now {
        //     return;
        // }

        let mid_now = (l_now + r_now) / 2;

        if self.lazy_adder[idx_now] != 0 {
            let adder = self.lazy_adder[idx_now];

            self.lazy_adder[idx_now * 2] += adder;
            self.lazy_adder[idx_now * 2 + 1] += adder;

            self.tree[idx_now * 2] += adder * (mid_now - l_now + 1) as i64;
            self.tree[idx_now * 2 + 1] += adder * (r_now - (mid_now + 1) + 1) as i64;

            self.lazy_adder[idx_now] = 0;
        }
    }

    pub fn add_on_range(
        &mut self,
        l: usize, // 总的范围
        r: usize,
        adder: i64,
        l_now: usize,
        r_now: usize,
        idx_now_tree: usize,
    ) {
        if l <= l_now && r_now <= r {
            // 包含了单点的情况.

            let count = r_now - l_now + 1;
            self.tree[idx_now_tree] += adder * count as i64;
            self.lazy_adder[idx_now_tree] += adder;
        } else {
            Self::down(self, l_now, r_now, idx_now_tree);
            let mid = (l_now + r_now) / 2;

            // 左侧需要进一步处理:
            if l <= mid {
                Self::add_on_range(self, l, r, adder, l_now, mid, idx_now_tree * 2);
            }
            if r > mid {
                Self::add_on_range(self, l, r, adder, mid + 1, r_now, idx_now_tree * 2 + 1);
            }

            // 这里, 孩子已经更新之后:
            self.tree[idx_now_tree] = self.tree[idx_now_tree * 2] + self.tree[idx_now_tree * 2 + 1];
        }
    }

    // 单点 修改
    pub fn sigle_asd(
        &mut self,
        adder: i64,
        target_idx: usize,
        l_now: usize,
        r_now: usize,
        idx_now: usize,
    ) {
        if l_now == r_now {
            self.tree[idx_now] += adder;
            return;
        }

        let mid = (l_now + r_now) / 2;
        if target_idx <= mid {
            Self::sigle_asd(self, adder, target_idx, l_now, mid, idx_now * 2);
        } else {
            Self::sigle_asd(self, adder, target_idx, mid + 1, r_now, idx_now * 2 + 1);
        }

        self.tree[idx_now] = self.tree[idx_now * 2] + self.tree[idx_now * 2 + 1];
    }

    pub fn sum_of_range(
        &mut self,
        l: usize, // 需要的l
        r: usize,
        l_now: usize, // 现在的l
        r_now: usize,
        idx_now_tree: usize,
    ) -> i64 {
        if l_now == r_now {
            return self.tree[idx_now_tree];
        }

        if l == l_now && r == r_now {
            return self.tree[idx_now_tree];
        }

        if l <= l_now && r_now <= r {
            return self.tree[idx_now_tree];
        }

        let mut sum = 0;

        let mid = (l_now + r_now) / 2;

        Self::down(self, l_now, r_now, idx_now_tree);

        if mid >= l {
            sum += Self::sum_of_range(self, l, r, l_now, mid, idx_now_tree * 2);
        }
        if r > mid {
            sum += Self::sum_of_range(self, l, r, mid + 1, r_now, idx_now_tree * 2 + 1);
        }

        sum
    }
}

// 输入用

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::{main, slove};

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_8_time_out() {
        // main();
        let file: File = File::open("testcase/P3372_8.in").expect("无法打开文件");
        let reader: BufReader<File> = BufReader::new(file);
        let mut lines: std::io::Lines<BufReader<File>> = reader.lines();

        // 读取第一行
        let first_line = lines.next().unwrap().unwrap();
        let total_opr = first_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>()[1];

        let second_line = lines.next().unwrap().unwrap();
        let data = second_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut op_list = vec![];
        for op in 0..total_opr {
            let line = lines.next().unwrap().unwrap();
            let parts = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            op_list.push(parts);
        }

        let mut my_res: std::io::BufWriter<std::io::Stdout> = slove(data, op_list);

        let file = File::open("testcase/P3372_8.out").expect("无法打开文件");
        let reader = BufReader::new(file);
        let mut lines: std::io::Lines<BufReader<File>> = reader.lines();

        while lines.next().is_some() {
            let line = lines.next().unwrap().unwrap();
        }
    }
}
