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

        let weight = parts[0];
        let cost = parts[1];

        objs.push(vec![weight, cost]);
    }

    assert_eq!(objs.len(), n);

    let res = solute(objs, n, total_weight);
    println!("{}", res);
}

fn solute(objs: Vec<Vec<usize>>, n: usize, total_weight: usize) -> usize {
    let mut aux = 0;

    for line in &objs {
        aux = aux.max(line[0]);
    }

    // // 考虑前i个公司, 能买到j的weight的 最小花费.
    // let mut record = vec![vec![usize::MAX; total_weight + 1 + aux]; n + 1];
    // // record[0][0] = 0;
    // let mut setted = vec![vec![false; total_weight + 1 + aux]; n + 1];

    // // dp:
    // record[0][0] = 0;
    // for i in 1..=n {
    //     for j in 0..=(total_weight + aux) {
    //         record[i][j] = record[i - 1][j];
    //         if j >= objs[i - 1][0] && record[i][j - objs[i - 1][0]] != usize::MAX {
    //             record[i][j] = record[i][j].min(objs[i - 1][1] + record[i][j - objs[i - 1][0]]);
    //         }
    //         setted[i][j] = true;
    //     }
    // }

    // find(&mut record, &mut setted, n, total_weight + aux, &objs);

    // for l in &record {
    //     println!("{:?}", l);
    // }

    // println!("last line: {:?}", &record[n]);

    // println!("first line: {:?}", &record[0]);

    // 考虑前i个公司, 能买到j的weight的 最小花费.
    let mut record2 = vec![vec![usize::MAX; total_weight + 1 + aux]; n + 1];
    let mut setted2 = vec![vec![false; total_weight + 1 + aux]; n + 1];

    for k in 0..=aux {
        find(&mut record2, &mut setted2, n, total_weight + k, &objs);
    }

    // find(&mut record2, &mut setted2, n, total_weight + aux, &objs);

    // for i in 0..=n {
    //     for j in 0..total_weight + aux + 1 {
    //         if setted[i][j] != setted2[i][j] {
    //             println!(
    //                 "设置 {} \t {} \t {:?}\t  {:?}\t",
    //                 i, j, setted[i][j], setted2[i][j],
    //             );
    //         }

    //         if record[i][j] != record2[i][j] {
    //             println!(
    //                 "数值 {} \t {} \t {:?}\t  {:?}\t",
    //                 i, j, record[i][j], record2[i][j]
    //             );
    //             // break;
    //         }
    //     }
    // }

    let mut res = usize::MAX;

    for idx in total_weight..(total_weight + 1 + aux) {
        res = res.min(record2[n][idx]);
    }

    res
}

fn find(
    record: &mut Vec<Vec<usize>>,
    setted: &mut Vec<Vec<bool>>,
    len: usize,
    remainj: usize,
    objs: &Vec<Vec<usize>>,
) -> usize {
    if setted[len][remainj] == true {
        return record[len][remainj];
    }

    if len == 0 {
        if remainj == 0 {
            record[len][remainj] = 0;
            setted[len][remainj] = true;
        } else {
            record[len][remainj] = usize::MAX;
            setted[len][remainj] = true;
        }

        return record[len][remainj];
    }

    // do not pick any len-1 object
    let mut res = find(record, setted, len - 1, remainj, objs);

    // let weight = objs[len - 1][0];
    // let cost = objs[len - 1][1];

    // pick some len-1 object
    if remainj >= objs[len - 1][0]
        && find(record, setted, len, remainj - objs[len - 1][0], objs) != usize::MAX
    {
        let tmp = find(record, setted, len, remainj - objs[len - 1][0], objs);

        res = res.min(tmp + objs[len - 1][1]);

        // println!("cost + cost: {} {}", cost, tmp,);
    }

    record[len][remainj] = res;
    setted[len][remainj] = true;

    // dp:
    // record[0][0] = 0;
    // for i in 1..=n {
    //     for j in 0..=(total_weight + aux) {
    //         record[i][j] = record[i - 1][j];
    //         if j >= objs[i - 1][0] && record[i][j - objs[i - 1][0]] != usize::MAX {
    //             record[i][j] = record[i][j].min(objs[i - 1][1] + record[i][j - objs[i - 1][0]]);
    //         }
    //     }
    // }

    record[len][remainj]
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
        let mut file = File::open("testcase/P2918_2.in").expect("Failed to open test input file");
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
    }
}
