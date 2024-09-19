struct RecentCounter {
    time_line: Vec<i32>,
    time_limit: i32,
    old: i32,
    new: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        return RecentCounter {
            time_line: vec![],
            time_limit: 3000,
            old: 0 - 3000,
            new: 0,
        };
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.time_line.push(t);
        self.new = t;
        self.old = t - self.time_limit;

        // println!("加入后 {:?}", self.time_line);

        // 删去过时的元素
        let mut index = 0usize;
        for i in 0..self.time_line.len() {
            if self.time_line[i] < self.old {
                index += 1;
            } else {
                break;
            }
        }
        // 删去[0..index]
        self.time_line.drain(0..index);


        // println!("截取后 {:?}", self.time_line);

        self.time_line.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

fn main() {
    let mut s = RecentCounter::new();

    println!("{}", s.ping(1));
    println!("{}", s.ping(100));
    println!("{}", s.ping(3001));
    println!("{}", s.ping(3002));

}
