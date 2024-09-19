// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Write, BufWriter};

// fn main() -> io::Result<()> {
//     // Step 1: 从 input.txt 中读取输入
//     let input_file = File::open("input.txt")?;
//     let input_reader = BufReader::new(input_file);

//     // 假设我们期望读取的输入是一行数字，解析成数组
//     let mut input_data: Vec<i32> = Vec::new();
//     for line in input_reader.lines() {
//         let line = line?;
//         for num in line.split_whitespace() {
//             input_data.push(num.parse().expect("无法解析数字"));
//         }
//     }

//     // Step 2: 计算结果（这部分根据实际需求修改）
//     let mut results: Vec<i32> = Vec::new();
//     for num in &input_data {
//         results.push(num * 2); // 示例逻辑：将每个输入数字乘以 2
//     }

//     // Step 3: 将结果写入到 BufWriter（指向 stdout）
//     let stdout = io::stdout();
//     let handle = stdout.lock();
//     let mut writer = BufWriter::new(handle);

//     // 写入结果并存储在结果 buffer 中
//     for result in &results {
//         writeln!(writer, "{}", result)?;
//     }

//     // Step 4: 读取 output.txt 作为正确答案
//     let output_file = File::open("output.txt")?;
//     let output_reader = BufReader::new(output_file);
//     let output_lines: Vec<String> = output_reader.lines().collect::<Result<_, _>>()?;

//     // Step 5: 验证每行结果与正确答案逐行对比
//     let mut is_correct = true;
//     for (i, result) in results.iter().enumerate() {
//         if i >= output_lines.len() {
//             println!("错误：输出行数超过正确答案的行数！");
//             is_correct = false;
//             break;
//         }
//         let expected_output: i32 = output_lines[i].trim().parse().expect("无法解析答案数字");
//         if *result != expected_output {
//             println!("错误：第 {} 行不匹配，结果是 {}，但期望的是 {}", i + 1, result, expected_output);
//             is_correct = false;
//         }
//     }

//     if is_correct {
//         println!("验证通过，所有结果均正确！");
//     } else {
//         println!("验证失败，有不匹配的结果。");
//     }

//     Ok(())
// }

/*

帮我写一个解题模板, rust, 从input.txt中得到输入的数组, 然后得到了一些结果,这些结果保存到一个bufWriter<stdout>中(方便交给oj平台),    同时我需要每一行验证与output.txt中的答案对比每一行, 看看是否正确.  我现在的困难是没法比较bufWriter<stdout>和答案文件之间一行一行对比结果.

兄弟, 我想请你比较bufWriter<stdout>和bufReader<File>的每一行, 而不是results这个vec, 有这个可能吗, 如果不能就算了?

*/

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Write};

fn main() -> io::Result<()> {
    // Step 1: 从 input.txt 中读取输入
    let input_file = File::open("input.txt")?;
    let input_reader = BufReader::new(input_file);

    // 假设我们期望读取的输入是一行数字，解析成数组
    let mut input_data: Vec<i32> = Vec::new();
    for line in input_reader.lines() {
        let line = line?;
        for num in line.split_whitespace() {
            input_data.push(num.parse().expect("无法解析数字"));
        }
    }

    // Step 2: 计算结果（这部分根据实际需求修改）
    let mut results: Vec<i32> = Vec::new();
    for num in &input_data {
        results.push(num * 2); // 示例逻辑：将每个输入数字乘以 2
    }

    // Step 3: 将结果写入到内存缓冲区（而不是直接写到 stdout）
    let mut buffer = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        for result in &results {
            writeln!(writer, "{}", result)?; // 将结果写入内存缓冲区
        }
    }

    // 将缓冲区转化为字符串，方便逐行比较
    let buffer_cursor = Cursor::new(buffer);
    let buffer_reader = BufReader::new(buffer_cursor);

    // Step 4: 读取 output.txt 作为正确答案
    let output_file = File::open("output.txt")?;
    let output_reader = BufReader::new(output_file);

    // Step 5: 比较每行结果与正确答案逐行对比
    let mut is_correct = true;

    for (output_line, result_line) in output_reader.lines().zip(buffer_reader.lines()) {
        let output_line = output_line?;
        let result_line = result_line?;

        if output_line.trim() != result_line.trim() {
            println!(
                "错误：预期输出 '{}' 与结果 '{}' 不匹配",
                output_line, result_line
            );
            is_correct = false;
        }
    }

    if is_correct {
        println!("验证通过，所有结果均正确！");
    } else {
        println!("验证失败，有不匹配的结果。");
    }

    Ok(())
}
