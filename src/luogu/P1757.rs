use std::{
    collections::HashMap,
    io::{self, *},
};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = parts[1];
    let total_weight = parts[0];

    let mut objs = Vec::new();

    // 读取接下来的 M 行
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let time = parts[0];
        let value = parts[1];
        let part_idx = parts[2];

        objs.push((time, value, part_idx));
    }

    assert_eq!(objs.len(), n);

    // 分组
    // objs.sort_by(|a, b| a.2.cmp(&b.2));
    let mut group_map = HashMap::new();

    // for ojb in objs {
    //     group_map.entry(ojb.2).or_insert(vec![]).push(ojb)
    // }

    //
    // 获取对应的Vec，如果不存在则创建一个新的Vec
    // map.entry(key).or_insert_with(Vec::new).push(num);

    for (idx, obj) in objs.iter().enumerate() {
        group_map.entry(obj.2).or_insert(vec![]).push(idx);
    }

    let mut obj_by_group = vec![];
    for (k, v) in group_map {
        let mut tmp = vec![];
        for vv in v {
            tmp.push(objs[vv]);
        }
        obj_by_group.push(tmp);
    }

    let mut record = vec![vec![0; total_weight + 1]; obj_by_group.len() + 1];
    let mut setted = vec![vec![false; total_weight + 1]; obj_by_group.len() + 1];

    find(
        obj_by_group.len(),
        total_weight,
        &mut record,
        &mut setted,
        &obj_by_group,
    );

    // println!("{:?}", obj_by_group);

    println!("{}", record[obj_by_group.len()][total_weight]);
}

fn find(
    len_group_limit: usize,
    remain_weight: usize,
    record: &mut Vec<Vec<usize>>,
    setted: &mut Vec<Vec<bool>>,
    obj_by_group: &Vec<Vec<(usize, usize, usize)>>,
) -> usize {
    if len_group_limit == 0 {
        return 0;
    }

    if setted[len_group_limit][remain_weight] == false {
        let mut max_now = 0;

        // 不选择 第id组

        max_now = find(
            len_group_limit - 1,
            remain_weight,
            record,
            setted,
            obj_by_group,
        );

        // 选择 第idx组的某一个
        for &obj in &obj_by_group[len_group_limit - 1] {
            let my_weight = obj.0;
            let my_value = obj.1;

            if remain_weight >= my_weight {
                let tmp = find(
                    len_group_limit - 1,
                    remain_weight - my_weight,
                    record,
                    setted,
                    obj_by_group,
                );

                max_now = max_now.max(my_value + tmp);
            }
        }

        record[len_group_limit][remain_weight] = max_now;
        setted[len_group_limit][remain_weight] = true;
    }
    record[len_group_limit][remain_weight]
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn it_works() {
        /*
         45 3
         10 10 1
         50 400 2
         10 5 1

        */
        main()
    }
}
