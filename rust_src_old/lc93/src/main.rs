struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let char_list: Vec<char> = s.chars().collect();
        let mut res = vec![];
        let mut r = vec![];
        let mut one = vec![
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ];

        Self::find(&mut r, &mut one, &char_list, 0, 0);

        for o in r {
            // println!("{:?}", o);
            //     let result = parts.join(".");
            res.push( o.join(".") );
        }

        res
    }
    pub fn find(
        res: &mut Vec<Vec<String>>,
        one: &mut Vec<String>,
        char_list: &Vec<char>,
        char_index: usize,
        segment_index: usize,
    ) {
        if char_index == char_list.len()  {
            if segment_index == 3 && char_index == char_list.len() && Self::check_ipv4(one.to_vec()) {

                res.push(one.to_vec());
            }

            return;
        }

        // 新的数字放到这一段
        one[segment_index].push(char_list[char_index]);
        Self::find(res, one, char_list, char_index + 1, segment_index);
        one[segment_index].pop();
        
        // 或者   新的数字放到下一段.  
        // 前一段是空的时候，不能选择把新的数字放入下一段： 前一段要不为空。
        if segment_index != 3  {

            one[segment_index + 1].push(char_list[char_index]);
            Self::find(res, one, char_list, char_index + 1, segment_index + 1);
            one[segment_index + 1].pop();
        }
    }

    fn check_ipv4(one: Vec<String>) -> bool {
        // 小于256  只包含数字   不包含前导0 
        for seg in one {
            if seg.len() == 0 {
                return false 
            }
            let num: usize = seg.parse().expect("msg");
            let re_str = num.to_string();

            if num > 255 {
                return false 
            }
            if re_str != seg {
                return false 
            }
        }
        true 
    }
}

fn main() {
    println!("Hello, world!");

    println!(
        "{:?}",
        Solution::restore_ip_addresses(String::from("101023"))
    )
}
