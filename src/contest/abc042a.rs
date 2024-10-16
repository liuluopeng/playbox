use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let mut syllables: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut result_buffer = Vec::new();
    slove(&mut result_buffer, &mut syllables);
    let output = String::from_utf8(result_buffer).expect("无法解析为 UTF-8 字符串");

    println!("{}", output);
}

pub fn slove(result_buffer: &mut Vec<u8>, syllables: &mut Vec<i32>) {
    let mut buf_writer = BufWriter::new(result_buffer);

    syllables.sort();
    if *syllables == vec![5, 5, 7] {
        writeln!(buf_writer, "YES").unwrap();
    } else {
        writeln!(buf_writer, "NO").unwrap();
    }

    buf_writer.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn it_works() {
        main();
    }
}
