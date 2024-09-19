use core::time;
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
    let time_limit = parts[0];
    let m = parts[1];

    let mut herbs = Vec::new(); // (time, value)

    // 读取接下来的 M 行
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let time = parts[0];
        let value = parts[1];
        herbs.push((time, value));
    }

    // println!("herbs: {:?}", herbs);

    // [i][j]: 前几个任意选择任意次, 不要超过j的时间
    let mut record = vec![vec![0; time_limit + 1]; m + 1];
    let mut setted = vec![vec![false; time_limit + 1]; m + 1];

    find(&mut record, &mut setted, m, time_limit, &herbs);

    // for l in &record {
    //     println!("{:?}", l);
    // }

    println!("{}", record[m][time_limit]);
}
fn find(
    record: &mut Vec<Vec<usize>>,
    setted: &mut Vec<Vec<bool>>,
    len_limit: usize,
    remain_time: usize,
    herbs: &Vec<(usize, usize)>,
) -> usize {
    if len_limit == 0 {
        return 0;
    }

    if setted[len_limit][remain_time] == false {
        let mut curr = 0;

        let mut my_time = herbs[len_limit - 1].0;
        let mut my_value = herbs[len_limit - 1].1;
        // case 1
        curr = 0 * my_value + find(record, setted, len_limit - 1, remain_time, herbs);

        // // case 2
        // while remain_time >= my_time {
        //     // let tmp = my_value + find(record, setted, len_limit - 1, remain_time - my_time, herbs);

        //     // 注意: 下一次
        //     let tmp = my_value + find(record, setted, len_limit, remain_time - my_time, herbs);

        //     curr = curr.max(tmp);

        //     my_value += my_value;
        //     my_time += my_time;
        // }

        // 注意: 下一次

        if remain_time >= my_time {
            let tmp = my_value + find(record, setted, len_limit, remain_time - my_time, herbs);

            curr = curr.max(tmp);
        }

        record[len_limit][remain_time] = curr;
        setted[len_limit][remain_time] = true;
    }

    record[len_limit][remain_time]
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn it_works() {
        /*

        70 3
        71 100
        69 1
        1 2


                         */
        main()
    }
}
