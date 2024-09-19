use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..n {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
        word = word.trim().to_string();

        // 在这里进行问题的解决逻辑
        if word.len() <= 10 {
            println!("{}", word);
        } else {
            let chars: Vec<char> = word.chars().collect();
            println!(
                "{}",
                chars[0].to_string()
                    + &(chars.len() - 2).to_string()
                    + &chars[chars.len() - 1].to_string()
            )
        }

        // 输出结果
        // println!("word: {}\n", word);
    }
}
