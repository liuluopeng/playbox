struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut res = vec![];

        let ss: Vec<char> = s.chars().collect();

        Self::find(&mut res, String::from(""), 0, ss);

        // for i in &res {
        //     println!("{:?}", i);
        // }

        res
    }

    pub fn find(res: &mut Vec<String>, turn: String, start_index: usize, ss: Vec<char>) {
        if ss.len() == start_index {
            // println!("到了  {:?}", turn);
            res.push(turn);

            return;
        }

        let mut turn = turn.clone();
        let char = ss[start_index];

        if !char.is_alphabetic() {
            // 不需要处理

            turn.push(char);
            Self::find(res, turn.clone(), start_index + 1, ss.clone());
            turn.pop();
        } else {
            // 分别大写和小写
            turn.push(char.to_ascii_lowercase());
            Self::find(res, turn.clone(), start_index + 1, ss.clone());
            turn.pop();

            turn.push(char.to_ascii_uppercase());
            Self::find(res, turn.clone(), start_index + 1, ss.clone());
            turn.pop();
        }
    }
}

fn main() {
    println!("Hello, world!");

    Solution::letter_case_permutation(String::from("a1b2"));

    // Solution::letter_case_permutation(String::from("123"));
}
