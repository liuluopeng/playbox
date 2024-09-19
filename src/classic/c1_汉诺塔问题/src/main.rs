struct Solution;

impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        let length = a.len();

        Self::rec(a, b, c, length, "a", "c", "b");

        println!("{:?} {:?} {:?} ", a, b, c);
    }
    pub fn rec(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>, length: usize, from_plate: &str, to_plate: &str, unused_plate: &str) {
        if length == 0 {
            return;
        }

        // 把除了最大的全都移动到B
        Self::rec(a, c, b, length - 1, "a", "b", "c");

        // 此时A只剩下一个，直接移动到 C
        let plate = a.pop().unwrap();
        c.push(plate);
        println!("从{} 移动 {:?} 到 {}", from_plate, plate, to_plate);

        // 现在B上面有n-1个   放到C上去
        Self::rec(b, a, c, length - 1, "b", "c", "a");
    }

    // 竖着打印 下标小的位置放的是较大的盘子
    pub fn pretty_print(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {}
}

fn main() {
    println!("Hello, world!");

    let mut a = vec![3, 2, 1];
    let mut b = vec![];
    let mut c = vec![];

    Solution::hanota(&mut a, &mut b, &mut c);
}
