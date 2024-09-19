use std::{mem, thread::panicking};

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut path_now = vec![vec![0, 0]];
        let mut memory = vec![];
        for i in 0..m {
            let mut line = vec![];
            for j in 0..n { 
                line.push(0);
            }
            memory.push(line);
        }
        let res = Self::find(m, n,  &mut path_now, &mut memory);

        
        for line in memory {
            println!("{:?}", line);
        }
        res 
    }

    pub fn find(m: i32, n: i32, path_now: &mut Vec<Vec<i32>>, memory: &mut Vec<Vec<i32>>) -> i32 {
        let row_now =  path_now[path_now.len() - 1][0];
        let col_now = path_now[path_now.len() - 1 ][1];
        

        if row_now == m - 1 && col_now == n - 1 {
            return 1;  // 1的意思是累加一条路径
        }

        if memory[row_now as usize][col_now as usize] != 0 {
            return memory[row_now as usize][col_now as usize];
        }


        let mut sum = 0;

        let mut path = path_now.clone();

        let length = path.len();
        let last_coo = path[length - 1].clone();


    

        // 向右边走一格
        let row_index = last_coo[0];
        let col_index = last_coo[1] + 1;
    
        if col_index < n {
            path.push(vec![row_index, col_index]);

            sum += Self::find(m, n,  &mut path, memory);
        }

        // 向下边走一格
        let row_index = last_coo[0] + 1;
        let col_index = last_coo[1];
        if row_index < m {
            path.push(vec![row_index, col_index]);   

            sum += Self::find(m, n,  &mut path, memory);
        }

        memory[row_now as usize][col_now as usize]  = sum;
        return sum;
    }
}
fn main() {
    println!("Hello, world!");
    println!("{}", Solution::unique_paths(7, 3));
}
