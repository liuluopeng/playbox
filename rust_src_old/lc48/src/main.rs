struct Solution;


impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let origin = matrix.clone();
        let n = origin.len();

        for i in 0..origin.len() {
            for j in 0..origin.len() {
                matrix[j][n - i - 1] = origin[i][j];
            }
        }

    }
}



fn main() {
    println!("Hello, world!");

    let mut  matrix = vec![  
        vec![1, 2, 3],  
        vec![4, 5, 6],  
        vec![7, 8, 9],  
    ];  

    println!("{:?}", Solution::rotate(&mut matrix))
}
