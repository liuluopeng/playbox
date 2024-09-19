use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = vec![];

        let mut keyboard = HashMap::new();
        keyboard.insert("2", vec!["a", "b", "c"]);
        keyboard.insert("3", vec!["d", "e", "f"]);
        keyboard.insert("4", vec!["g", "h", "i"]);
        keyboard.insert("5", vec!["j", "k", "l"]);
        keyboard.insert("6", vec!["m", "n", "o"]);
        keyboard.insert("7", vec!["p", "q", "r", "s"]);
        keyboard.insert("8", vec!["t", "u", "v"]);
        keyboard.insert("9", vec!["w", "x", "y", "z"]);

        let digits_list: Vec<char> = digits.chars().collect();

        if digits_list.len() == 0 {
            return vec![];
        }

        Self::find(
            digits_list,
            &mut 0,
            &mut String::from(""),
            &mut res,
            keyboard,
        );

        res
    }

    pub fn find(
        digits_list: Vec<char>,
        index_now: &mut usize,
        this_turn_now: &mut String,
        res: &mut Vec<String>,
        keyboard: HashMap<&str, Vec<&str>>,
    ) {
        if *index_now >= digits_list.len() {
            // println!("所有到达return的结果  {:?} {:?}", this_turn_now, *index_now);

            if *index_now == digits_list.len()  {
                res.push(this_turn_now.to_string());
            }
            return;
        }

        let keys: &Vec<&str> = keyboard
            .get(digits_list[*index_now].to_string().as_str())
            .unwrap();

        // println!("{:?}", keys);

        for letter in keys {
            // println!("{:?}", letter);

            let mut this_turn_now = this_turn_now.clone();
            this_turn_now.push_str(letter);
            // println!("{:?}", this_turn_now);

            *index_now += 1;
            Self::find(
                digits_list.clone(),
                index_now,
                &mut this_turn_now.clone(),
                res,
                keyboard.clone(),
            );
            *index_now -= 1;
        }
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", Solution::letter_combinations(String::from("23")))
}
