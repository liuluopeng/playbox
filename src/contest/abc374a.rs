use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    // 读取第一行
    let a = lines.next().unwrap().unwrap();

    let mut result_buffer = Vec::new();

    if a.ends_with("san") {
        writeln!(result_buffer, "Yes").unwrap();
    } else {
        writeln!(result_buffer, "No").unwrap();
    }

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
