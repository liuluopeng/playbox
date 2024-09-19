struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut res = vec![];

        let mut size = 1;
        // 寻找合适的范围
        loop {
            if size < label {
                size *= 2;
            } else {
                break;
            }
        }

        // let mut list: Vec<i32> = (1..size).collect();
        let mut this_level_size = 1;
        let mut left_direc = true;

        let mut list = vec![];
        let index = 0;
        let mut be_pushed = 1;
        while list.len() < size.try_into().unwrap() {
            if left_direc {
                // 从左向右填充
                for i in 0..this_level_size {
                    list.push(be_pushed);
                    be_pushed += 1;
                }
                be_pushed -= 1;
            } else {
                be_pushed += this_level_size;
                for i in 0..this_level_size {
                    list.push(be_pushed);
                    be_pushed -= 1;
                }
                be_pushed += 1;
                be_pushed += this_level_size;
            }

            left_direc = !left_direc;
            this_level_size *= 2;
        }


        let list = list.clone();
        let mut find_index = 0;
        for i in 0..list.len() {
            if list[i]==label {
                find_index = i;
            }
        }


        while find_index > 0 {
            res.push(list[find_index as usize]);
            if find_index % 2 == 0 { // 右子树
                find_index = find_index/2 - 1;
            } else {
                find_index /= 2;
            }
        }

        res.push(1);
        res.reverse();

        res
    }
}

fn main() {
    println!("Hello, world!");

    let label = 26;

    Solution::path_in_zig_zag_tree(label);
}
