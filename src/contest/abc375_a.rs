use std::io::{self, *};

pub fn main() {
    let mut result_buffer = Vec::new();
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    let a = lines.next().unwrap().unwrap();
    let a = lines.next().unwrap().unwrap();

    let chs: Vec<char> = a.chars().collect();

    let mut cnt = 0;

    if chs.len() < 3 {
    } else {
        for k in 0..=chs.len() - 3 {
            if chs[k] == '#' && chs[k + 2] == '#' && chs[k + 1] == '.' {
                cnt += 1;
            }
        }
    }

    // let mut k = 0;

    // while k + 3 >= chs.len() {
    //     if chs[k] == '#' && chs[k + 2] == '#' && chs[k + 1] == '.' {
    //         cnt += 1;
    //     }
    //     k += 1;
    // }

    writeln!(result_buffer, "{}", cnt).unwrap();

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
