use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let number = parts[0];
    let exp = parts[1];
    let modder: usize = parts[2];

    // output: 2^10 mod 9=7

    let mut res = 1;

    let mut now_time = 0; // 一开始是2的1次
                          // let mut now_time = 1; // 一开始是2的1次

    let mut my_a = number;
    while now_time <= 31 {
        // println!("res: {} {} {}", res, now_time, 1 << now_time);

        if exp & (1 << now_time) != 0 {
            // println!("nowtime{} is 1", now_time);

            // res = res * number.pow(1 << now_time);
            // res = res % modder;

            res = (res * my_a) % modder;
        }

        my_a = (my_a * my_a) % modder;
        now_time += 1;
    }

    println!("{}^{} mod {}={}", number, exp, modder, res);
}

#[cfg(test)]
mod tests {

    use super::main;

    #[test]
    fn it_works() {
        main();
    }
}
