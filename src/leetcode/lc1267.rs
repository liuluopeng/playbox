impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut counter = 0;
        let mut coo1d_list = vec![];

        let m = grid.len();
        let n = grid[0].len();

        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[0].len() {
                if grid[row_idx][col_idx] == 1 {
                    counter += 1;
                    coo1d_list.push(row_idx * n + col_idx);
                }
            }
        }

        let mut uf = UnionFind::new(counter);

        for i in 0..coo1d_list.len() {
            for j in 0..coo1d_list.len() {
                let row_i = coo1d_list[i] / n;
                let col_i = coo1d_list[i] % n;
                let row_j = coo1d_list[j] / n;
                let col_j = coo1d_list[j] % n;

                if row_i == row_j {
                    uf.union(i, j);
                }
                if col_i == col_j {
                    uf.union(i, j);
                }
            }
        }

        let mut alone_counter = 0;
        for idx in 0..counter {
            if uf.is_alone(idx) {
                alone_counter += 1;
            }
        }

        println!("uf set number: {}  alone: {} counter: {}", uf.get_counter(), alone_counter, counter);

        (counter - alone_counter) as i32
    }
}
use crate::{solution::Solution, union_find::UnionFind};

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            Solution::count_servers(vec_2d_leetcode("[[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]"))
        );
    }
}
