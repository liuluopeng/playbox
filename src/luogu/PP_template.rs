use std::io::{self, *};

pub fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let total_opr: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // res_writer.flush().unwrap();

    let mut result_buffer = Vec::new();

    slove(&mut result_buffer);
    let output = String::from_utf8(result_buffer).expect("无法解析为 UTF-8 字符串");

    // println!("{:?}", res_writer);
    println!("{}", output);
}

pub fn slove(result_buffer: &mut Vec<u8>) {
    let mut buf_writer = BufWriter::new(result_buffer);

    writeln!(buf_writer, "hello").unwrap();

    buf_writer.flush().unwrap();

    writeln!(buf_writer, "world!").unwrap();

    buf_writer.flush().unwrap();
}

// 输入用

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Cursor, Write},
    };

    use super::{main, slove};

    #[test]
    /// 直接执行, 手动粘贴.
    fn test_main() {
        main();
    }

    #[test]
    fn test_1() {
        let answer_input_path = "testcase/PP_template.in";
        let answer_output_path = "testcase/PP_template.out";

        let file: File = File::open(answer_input_path).expect("无法打开文件");
        let reader: BufReader<File> = BufReader::new(file);

        // main start:
        let mut lines: std::io::Lines<BufReader<File>> = reader.lines();
        // 读取第一行
        let first_line = lines.next().unwrap().unwrap();
        let total_opr = first_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>()[1];

        let mut buf = vec![];
        let mut my_res = slove(&mut buf);

        // main end. 从下面开始是 和 题目的输入 无关的 .

        // 将缓冲区转化为字符串，方便逐行比较
        let buffer_cursor = Cursor::new(buf);
        let buffer_reader = BufReader::new(buffer_cursor);

        // Step 4: 读取 output.txt 作为正确答案
        let output_file = File::open(answer_output_path).unwrap();
        let output_reader = BufReader::new(output_file);

        // Step 5: 比较每行结果与正确答案逐行对比
        let mut is_correct = true;

        for (output_line, result_line) in output_reader.lines().zip(buffer_reader.lines()) {
            let output_line = output_line.unwrap();
            let result_line = result_line.unwrap();

            if output_line.trim() != result_line.trim() {
                println!(
                    "错误：预期输出 '{}' 与结果 '{}' 不匹配",
                    output_line, result_line
                );
                is_correct = false;
            }

            assert_eq!(output_line.trim(), result_line.trim());
        }

        if is_correct {
            println!("验证通过，所有结果均正确！");
        } else {
            println!("验证失败，有不匹配的结果。");
        }
        println!("测试结束");
    }
}
