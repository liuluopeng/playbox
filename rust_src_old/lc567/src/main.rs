fn main() {
    println!("Hello, world!");

    let s1 = "adc".to_string();
    let s2 = "dcda".to_string();

    println!("{:?}", Solution::check_inclusion(s1, s2));
}
struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {

        

        // [a b c d ... z] [1,2,3, ... 4]  : a bb ccc ... zzzzzzzzzzzzzzzzzzzzzzzz

        let mut quick_counter = vec![0; 26];

        for b in s1.bytes() {
            // println!("{:?}", b);
            quick_counter[b as usize - b'a' as usize] += 1;
        }

        // println!("{:?}", quick_counter);

        let mut j = s1.len() - 1;

        // count s2[i..j]
        let mut s2_counter = vec![0; 26];
        let s2_chars: Vec<char> = s2.chars().collect();
        for k in 0..=j {
            s2_counter[s2_chars[k] as usize - 'a' as usize] += 1;
        }
        if s2_counter == quick_counter {
            return true;
        }
        for k in s1.len()..s2.len() {
            // println!("{:?}", s2_chars[k]);
            s2_counter[s2_chars[k] as usize - 'a' as usize] += 1;
            s2_counter[s2_chars[k - s1.len()] as usize - 'a' as usize] -= 1;

            // println!("{:?} \n{:?} \n", quick_counter, s2_counter);

            if s2_counter == quick_counter {
                return true;
            }
        }

        false
    }
}
