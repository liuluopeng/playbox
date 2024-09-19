struct Solution;
impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness.clone();
        happiness.sort();

        let mut res = 0;

        let mut addup = 0;
        for i in ((happiness.len() - k as usize)..happiness.len()).rev() {
            println!("{:?}", happiness[i]);

            if happiness[i] > addup {
                // happiness[i] - addup > 0
                res += (happiness[i] - addup) as i64;
            } else {
                res += 0;
            }
            addup += 1;
        }

        // println!("res: {:?}", res);

        res
    }
}
/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

fn main() {
    println!("Hello, world!");

    let hap = vec![1,1,1,1];
    let k = 4;
    println!("{:?}", Solution::maximum_happiness_sum(hap, k));
}
