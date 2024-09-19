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
    let t = parts[0];
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

    // 范围从 全选  到 全不选
    let mut record = vec![vec![0; t + 1]; m + 1]; // record[idx][time]: 前idx个草药,time内能取得的最大value
    let mut setted = vec![vec![false; t + 1]; m + 1];


    // for j in 0..

    let res = find(&mut record, &mut setted, &herbs, m, t, t);

    // for l in &record {
    //     println!("{:?}", l);
    // }

    println!("{}", res);
}

fn find(
    record: &mut Vec<Vec<usize>>,
    setted: &mut Vec<Vec<bool>>,
    herbs: &Vec<(usize, usize)>,
    slice_len: usize,
    time_dura_limit: usize,
    total_time_limit: usize,
) -> usize {
    if slice_len == 0 {
        return 0;
    }

    // if time_dura > time_limie {
    //     return 0;
    // }

    if setted[slice_len][time_dura_limit] == false {
        let mut do_not_pick = 0;
        let mut pick = 0;
        do_not_pick = find(
            record,
            setted,
            herbs,
            slice_len - 1,
            time_dura_limit,
            total_time_limit,
        );

        if time_dura_limit >= herbs[slice_len - 1].0 {
            pick = find(
                record,
                setted,
                herbs,
                slice_len - 1,
                time_dura_limit - herbs[slice_len - 1].0,
                total_time_limit,
            ) + herbs[slice_len - 1].1;
        }

        let res = pick.max(do_not_pick);
        record[slice_len][time_dura_limit] = res;
        setted[slice_len][time_dura_limit] = true;
    }
    // pick

    record[slice_len][time_dura_limit]
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
