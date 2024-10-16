impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut res = String::new();

        let char_list: Vec<char> = s.chars().collect();
        let done = Self::find(char_list);

        for c in done {
            res.push(c);
        }
        res
    }

    pub fn find(slice: Vec<char>) -> Vec<char> {
        let mut find_br = false;

        for i in 0..slice.len() {
            if slice[i] == '[' {
                find_br = true;
                break;
            }
        }
        if find_br {
            let mut pre: Vec<char> = vec![];

            for i in 0..slice.len() {
                if slice[i] < '0' || slice[i] > '9' {
                    pre.push(slice[i]);
                } else {
                    break;
                }
            }

            let mut time = 0;
            for i in pre.len()..slice.len() {
                if slice[i] >= '0' && slice[i] <= '9' {
                    time = time * 10 + (slice[i] as usize - '0' as usize);
                } else {
                    break;
                }
            }

            // println!("{:?} time", time);

            let mut start_idx = 0;
            let mut end_idx = 0;
            for i in 0..slice.len() {
                if slice[i] == '[' {
                    start_idx = i;
                    break;
                }
            }

            let mut stack = vec![];
            for i in start_idx..slice.len() {
                if slice[i] == '[' {
                    stack.push(i);
                }
                if slice[i] == ']' {
                    stack.pop();
                    if stack.is_empty() {
                        end_idx = i;
                        break;
                    }
                }
            }

            let mut middle: Vec<char> = vec![];
            for i in (start_idx + 1)..end_idx {
                middle.push(slice[i]);
            }

            // println!("sub question:{:?}", middle);
            let done_middle = Self::find(middle);
            let done_middle = done_middle.repeat(time);
            pre.extend(done_middle);

            let mut suf = vec![];
            for i in end_idx + 1..slice.len() {
                suf.push(slice[i]);
            }

            let sef_done = Self::find(suf);

            pre.extend(sef_done);

            return pre;
        } else {
            return slice;
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::leetcode::lc394::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            "accaccacc".to_string(),
            Solution::decode_string("3[a2[c]]".to_string())
        );

        assert_eq!(
            "abccdcdcdxyz".to_string(),
            Solution::decode_string("abc3[cd]xyz".to_string())
        );

        assert_eq!(
            "abcabccdcdcdef".to_string(),
            Solution::decode_string("2[abc]3[cd]ef".to_string())
        );
    }
}
