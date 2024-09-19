use regex::Regex;
use std::{
    clone,
    fs::{self, File},
    io::{BufRead, BufReader},
};

struct ATM {
    // 20 50 100 200 500
    remain_list: Vec<i64>,
    nominal_values: Vec<i64>,
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    fn new() -> Self {
        ATM {
            remain_list: vec![0; 5],
            nominal_values: vec![20, 50, 100, 200, 500],
        }
    }

    // 存钱
    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, count) in banknotes_count.iter().enumerate() {
            self.remain_list[i] += *count as i64;
        }
    }

    // 取钱
    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut amount = amount as i64;
        let mut res: Vec<i32> = vec![0, 0, 0, 0, 0];
        // 从大数额开始取钱
        let mut index = 4usize;
        loop {
            if self.remain_list[index] == 0 {
                if index == 0 {
                    break;
                }
                index -= 1;
                continue;
            }
            res[index] = (amount / self.nominal_values[index]) as i32;
            // 当没有这么多面额的纸币的时候， 取出全部的纸币。
            if res[index] > self.remain_list[index] as i32 {
                res[index] = self.remain_list[index] as i32;
            }
            amount -= res[index] as i64 * self.nominal_values[index];
            if index == 0 {
                break;
            }
            index -= 1;
        }

        // println!("应该取出  {:?}", res);

        // 检查余量是否满足
        for (i, count) in self.remain_list.iter().enumerate() {
            if (res[i]) as i64 > self.remain_list[i] {
                return vec![-1];
            }
        }

        if amount != 0 {
            return vec![-1];
        }

        // 减去库存
        for (i, count) in self.remain_list.iter_mut().enumerate() {
            *count -= res[i] as i64;
        }

        res
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i64> = obj.withdraw(amount);
 */

fn main() {
    println!("Hello, world!");

    // 打开文件
    let file = File::open("test_case.txt").expect("Failed to open file");

    // 使用BufReader包装文件
    let reader = BufReader::new(file);

    // 逐行读取文件内容
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect();

    // 打印每一行内容
    let actions: String = lines[0].clone();
    let data: String = lines[1].clone();

    let actions: Vec<&str> = actions
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .collect();

    let re = Regex::new(r"\[[^\[\]]*\]").unwrap();
    let data: Vec<&str> = re.find_iter(&data).map(|m| m.as_str()).collect();

    let mut atm = ATM::new();

    // let actions: Vec<&str> = vec!["ATM","deposit","withdraw"];

    for (i, &action) in actions.iter().enumerate() {
        // println!("{:?} ", action,);
        let action = &action[1..action.len() - 1];

        match action {
            "ATM" => {
                // 新建ATM
            }
            "deposit" => {
                // 存钱
                let input = data[i].trim_matches(|c| c == '[' || c == ']' || c == ' '); // 去除方括号和空格
                let banknotes_count: Vec<i32> =
                    input.split(',').filter_map(|s| s.parse().ok()).collect();
                atm.deposit(banknotes_count.clone());

                // println!("充钱 {:?}", banknotes_count);
            }
            "withdraw" => {
                // 取钱
                // println!("{:?}", data[i]);

                let amount: i32 = data[i][1..data[i].len() - 1].parse().expect("msg");
                let res = atm.withdraw(amount);
                println!(
                    "{} \t 取出 {:?}  \t 还剩下{:?}",
                    amount, res, atm.remain_list
                );
            }
            _ => {
                println!("匹配不到 {}", action);
                if action == "ATM" {
                    println!("{:?}", &data[i]);
                }
            }
        }
    }
}
