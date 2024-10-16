use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    // 读取第一行
    let first_line: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut points = vec![];
    for i in 0..first_line {
        let point: Vec<i64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        points.push(point);
    }

    let mut result_buffer = Vec::new();

    let mut last_point = vec![0, 0];

    let mut dist_sum = 0.0;

    // println!("{:?}", points);

    points.push(vec![0, 0]);

    for p in points {
        let dist = (p[0] - last_point[0]) * (p[0] - last_point[0])
            + (p[1] - last_point[1]) * (p[1] - last_point[1]);

        // println!("{:?}", dist);

        let dist = dist as f64;
        let dist = dist.sqrt();

        // println!("{}", dist);

        dist_sum += dist;
        last_point = p;
    }

    writeln!(result_buffer, "{}", dist_sum).unwrap();

    let output = String::from_utf8(result_buffer).expect("无法解析为 UTF-8 字符串");

    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn it_works() {
        main();
    }
}
