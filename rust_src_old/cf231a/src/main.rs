use std::io;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("msg1");
    // println!("{:?}", n_str);
    let length: i32 = n_str.trim().parse().expect("msg2");
    let mut count: usize = 0;

    for _i in 0..length {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("msg3");
        
        // println!("{:?}", line);
        let mut parts = line.trim().split_whitespace();

        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        let c: i32 = parts.next().unwrap().parse().unwrap();

        if a + b + c > 1 {
            count += 1;
        }
    }

    println!("{}", count);
}
