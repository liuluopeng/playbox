pub struct Solution;

// write my solution here:
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut start = 0;
        let mut end = 0;
        let mut max_now = 0;

        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            let mut one_turn = 1;

            if i != 0 && i != chars.len() - 1 {
                let mut j_left = i - 1;
                let mut j_right = i + 1;
                while j_right < chars.len() && chars[j_left] == chars[j_right] {
                    one_turn += 2;

                    if j_left == 0 {
                        break;
                    }
                    if j_right == chars.len() - 1 {
                        break;
                    }

                    j_left -= 1;
                    j_right += 1;
                }
            }

            if one_turn > max_now {
                max_now = one_turn;
                start = i - one_turn / 2;
                end = i + one_turn / 2;
            }

            if i != chars.len() - 1 && chars[i + 1] == chars[i] {
                // println!("偶数回文{:?}", chars[i]);
                let mut even_turn = 2;

                if i != 0 && i != chars.len() - 1 {
                    let mut j_left = i - 1;
                    let mut j_right = i + 2;
                    while j_right < chars.len() && chars[j_left] == chars[j_right] {
                        even_turn += 2;

                        if j_left == 0 {
                            break;
                        }
                        if j_right == chars.len() - 1 {
                            break;
                        }

                        j_left -= 1;
                        j_right += 1;
                    }
                }

                if even_turn > max_now {
                    max_now = even_turn;
                    start = i - even_turn / 2 + 1;
                    end = i + even_turn / 2;
                }
            }
        }

        let mut res = String::new();
        for i in 0..chars.len() {
            if i >= start && i <= end {
                res.push(chars[i]);
            }
        }

        println!("{:?}", max_now);

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let st = "babad".to_string();
        println!("{:?}", Solution::longest_palindrome(st));
    }
}
