use std::io::{self, *};

pub fn main() {
    let mut result_buffer = Vec::new();
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    let a = lines.next().unwrap().unwrap();
    let chs: Vec<char> = a.chars().collect();

    for ch in chs {
        if ch != '.' {
            write!(result_buffer, "{}", ch).unwrap();
        }
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
