fn main() {
    println!("Hello, world!");
}


struct Solution{}


impl Solution {
    
    pub fn average(salary: Vec<i32>) -> f64 {
        let l: i32 = salary.len() as i32;
        let mut min: i32 = 1_000_000;
        let mut max: i32 = 1_000;

        let mut sum = 0;
        for s in salary {
            if s > max {
                max = s;
            }
            if s < min {
                min = s;
            }

            sum += s;
        }


        ((sum - max - min) as f64 / (l - 2  )  as f64).into()
    }
}