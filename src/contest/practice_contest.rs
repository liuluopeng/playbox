use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    // 读取第一行
    let a: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    let bc: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let s = lines.next().unwrap().unwrap();

    let mut result_buffer = Vec::new();

    writeln!(result_buffer, "{} {}", a + bc[0] + bc[1], s).unwrap();

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
