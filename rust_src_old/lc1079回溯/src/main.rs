struct Solution;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut res = vec![];

        let tile_chars: Vec<char> = tiles.chars().collect();
        let mut this_turn: Vec<char> = vec![];
        Self::find(tile_chars, &mut 0, &mut this_turn, &mut res);

        for i in &res {
            println!("{:?}", i)
        }

        res.clone().len() as i32
    }

    pub fn find(
        tile_chars: Vec<char>,
        index: &mut usize,
        this_turn: &mut Vec<char>,
        res: &mut Vec<String>,
    ) {
        if *index == tile_chars.len() {
            res.push(this_turn.iter().collect());
            return;
        }

        this_turn.push(tile_chars[*index]);
        *index += 1;
        Self::find(tile_chars, index, this_turn, res);

        this_turn.pop();
        // *index -= 1;
    }
}

fn main() {
    println!("Hello, world!");
    println!(
        "{:?}",
        Solution::num_tile_possibilities(String::from("AAB"))
    )
}
