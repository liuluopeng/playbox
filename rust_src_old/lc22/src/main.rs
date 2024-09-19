struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];

        let mut now = String::from("");
        Self::dfs(n, &mut res, &mut now);
        res
    }

  
    pub fn dfs(n:i32, res: &mut Vec<String>, now: &mut String) {
        if now.len() as i32 == 2*n {
            if Self::check(now.to_string()) {
                res.push(now.to_string());

            }
            return
        }

        let mut now = now.clone();
        now.push('(');
        Self::dfs(n,res, &mut now);

        now.pop();
        now.push(')');
        Self::dfs(n, res, &mut now);
    }

    pub fn check(now: String) -> bool {
        let mut stack = vec![];
        for c in now.chars() {
            if c == '(' {
                stack.push(c);
            } else {
                if stack.is_empty() { // ")))"
                    return false 
                } else { // stack not empty
                    stack.pop();
                }

            }
        }
        
        stack.is_empty()
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", Solution::generate_parenthesis(3));
}
