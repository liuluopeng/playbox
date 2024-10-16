use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();
    let mut result_buffer = Vec::new();

    let aa: Vec<char> = a.chars().collect();
    let bb: Vec<char> = b.chars().collect();

    let mut cnt: i32 = 0;
    for i in 0..aa.len().min(bb.len()) {
        if aa[i] == bb[i] {
            cnt += 1;
        } else {
            break;
        }
    }

    if cnt == aa.len() as i32 && cnt == bb.len() as i32 {
        cnt = -1;
    }

    writeln!(result_buffer, "{}", cnt + 1).unwrap();
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
