use std::{
    io::{self, BufRead},
    usize,
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

    let n = parts[0];
    let total_weight = parts[1];

    let mut objs = Vec::new();

    // 读取接下来的 M 行
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let value = parts[0];
        let weight = parts[1];
        let count = parts[2];

        objs.push(vec![value, weight, count]);
    }

    assert_eq!(objs.len(), n);

    let res = solute(objs, n, total_weight);
    println!("{}", res);
}

fn solute(objs: Vec<Vec<usize>>, n: usize, total_weight: usize) -> usize {
    let mut res = 0;
    let mut new_bag = vec![];

    for obj in &objs {
        let value = obj[0];
        let weight = obj[1];
        let count = obj[2];

        let mut double = 1;
        let mut counter = count;
        // 创造出一些物品

        while double <= counter {
            new_bag.push(vec![double * value, double * weight]);

            counter -= double;
            double *= 2;
        }
        if counter > 0 {
            new_bag.push(vec![counter * value, counter * weight]);
        }
    }

    // let mut dp = vec![vec![0; total_weight + 1]; new_bag.len() + 1];
    // for i in 1..=new_bag.len() {
    //     for j in 0..total_weight + 1 {
    //         dp[i][j] = dp[i - 1][j];
    //         if j >= new_bag[i - 1][1] {
    //             dp[i][j] = dp[i][j].max(new_bag[i - 1][0] + dp[i - 1][j - new_bag[i - 1][1]]);
    //         }
    //     }
    // }

    // res = dp[new_bag.len()][total_weight];

    let mut line1 = vec![0; total_weight + 1];

    for i in 1..=new_bag.len() {
        for j in (0..total_weight + 1).rev() {
            if j >= new_bag[i - 1][1] {
                line1[j] = line1[j].max(new_bag[i - 1][0] + line1[j - new_bag[i - 1][1]]);
            }
        }
    }

    line1[line1.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, Read, Seek, SeekFrom, Write};
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

    fn redirect_stdin(input: &str) -> io::Result<(RawFd, File)> {
        // 创建一个临时文件以存储输入数据
        let mut temp_file = tempfile::tempfile()?;
        temp_file.write_all(input.as_bytes())?;
        temp_file.seek(SeekFrom::Start(0))?;

        // 获取当前标准输入的文件描述符
        let stdin_fd = io::stdin().as_raw_fd();

        // 复制原始标准输入文件描述符
        let original_stdin = unsafe { File::from_raw_fd(libc::dup(stdin_fd)) };

        // 重定向标准输入到临时文件
        unsafe {
            libc::dup2(temp_file.as_raw_fd(), stdin_fd);
        }

        Ok((stdin_fd, original_stdin))
    }

    fn redirect_stdout() -> io::Result<(RawFd, File)> {
        let stdout_fd = io::stdout().as_raw_fd();
        let original_stdout = unsafe { File::from_raw_fd(libc::dup(stdout_fd)) };
        let temp_file = tempfile::tempfile()?;

        unsafe {
            libc::dup2(temp_file.as_raw_fd(), stdout_fd);
        }

        Ok((stdout_fd, original_stdout))
    }

    fn restore_stdin(stdin_fd: RawFd, original_stdin: File) {
        // 恢复原始标准输入
        unsafe {
            libc::dup2(original_stdin.as_raw_fd(), stdin_fd);
        }
    }

    fn restore_stdout(stdout_fd: RawFd, original_stdout: File) {
        unsafe {
            libc::dup2(original_stdout.as_raw_fd(), stdout_fd);
        }
    }

    #[test]
    fn test_main() {
        // 读取测试输入文件内容
        let mut file = File::open("testcase/P1776_4.in").expect("Failed to open test input file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read test input file");

        // 重定向标准输入
        let (stdin_fd, original_stdin) =
            redirect_stdin(&contents).expect("Failed to redirect stdin");

        // 重定向标准输出
        // let (stdout_fd, original_stdout) = redirect_stdout().expect("Failed to redirect stdout");

        // 执行 main 函数
        main();

        // 恢复标准输入
        // restore_stdin(stdin_fd, original_stdin);

        // // 恢复标准输出并读取输出内容
        // let mut temp_file = unsafe { File::from_raw_fd(stdout_fd) };
        // let mut output = String::new();
        // temp_file
        //     .seek(SeekFrom::Start(0))
        //     .expect("Failed to seek stdout file");
        // temp_file
        //     .read_to_string(&mut output)
        //     .expect("Failed to read stdout");

        // restore_stdout(stdout_fd, original_stdout);

        // // 验证输出
        // println!("output: {:?}", output);
        // assert_eq!(output, "12345".to_string());

        // ---- luogu::P1776::tests::test_main stdout ---- 784573
    }
}
