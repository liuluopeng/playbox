struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut supply = vec![0; 26];

        for l in &letters {
            supply[(*l as u8 - b'a') as usize] += 1;
        }

        let res = Self::find(&words, &mut vec![], 0, &score, &supply);

        res
    }

    // 先找出所有的word子集。  然后每个子集看看能不能用letters拼写出来， 然后算分数。

    pub fn find(
        words: &Vec<String>,
        turn: &mut Vec<String>,
        start_index: usize,
        score: &Vec<i32>,
        supply: &Vec<i32>,
    ) -> i32 {
        let mut res = Self::score_of_turn(turn, score, supply);

        for i in start_index..words.len() {
            turn.push(words[i].clone());

            res = Self::max(res, Self::find(words, turn, i + 1, score, supply));

            turn.pop();
        }

        return res;
    }

    pub fn score_of_turn(turn: &Vec<String>, score: &Vec<i32>, supply: &Vec<i32>) -> i32 {
        let mut need = vec![0; 26];

        for word in turn {
            for char in word.chars() {
                need[(char as u8 - 'a' as u8) as usize] += 1;
            }
        }

        let mut res = 0;
        for i in 0..26 {
            if need[i] > supply[i] {
                return -1;
            }

            res += need[i] * score[i];
        }

        res
    }

    pub fn max(a: i32, b: i32) -> i32 {
        if a > b {
            return a;
        }
        b
    }
}

fn main() {
    println!("Hello, world!");

    let words = vec![
        String::from("dog"),
        String::from("cat"),
        String::from("dad"),
        String::from("good"),
    ];

    let letters: Vec<char> = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];

    let score = vec![
        1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let res = Solution::max_score_words(words, letters, score);
    println!("答案：{}", res);
}
