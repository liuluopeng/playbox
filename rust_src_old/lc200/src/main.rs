fn main() {
    println!("Hello, world!");

    // [["1","1","0","0","0"],["1","1","0","0","0"],["0","0","1","0","0"],["0","0","0","1","1"]]
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    println!("{:?}", Solution::num_islands(grid));
}

struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count: usize = 0;

        let m = grid.len();
        let n = grid[0].len();

        let mut exists = vec![];
        for i in 0..m {
            let line = vec![false; n];
            exists.push(line);
        }

        for i in 0..m {
            for j in 0..n {
                if exists[i][j] == false && grid[i][j] == '1' {
                    Solution::find(&grid, &mut count, m, n, &mut exists, i, j);
                    count += 1;
                }

                // println!();
            }
        }

        // Solution::find(&grid, &mut count, m, n, &mut exists, 0, 0);

        count as i32
    }

    pub fn find(
        grid: &Vec<Vec<char>>,
        count: &mut usize,
        m: usize,
        n: usize,
        exsits: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
    ) {
        // 以[i,j]为开始,  向四周寻找连在一起的方块.

        // exsits[i][j] = true;

        if exsits[i][j] == false {
            // println!("{:?} {:?} 正在访问", i, j);
            exsits[i][j] = true;
            for i_delta in [-1, 0, 1].iter().cloned() {
                for j_delta in [-1, 0, 1].iter().cloned() {
                    if i_delta == 0 && j_delta == 0 {
                        continue;
                    }

                    if i == 0 && i_delta == -1 {
                        continue;
                    }
                    if j == 0 && j_delta == -1 {
                        continue;
                    }
                    if i == m - 1 && i_delta == 1 {
                        continue;
                    }
                    if j == n - 1 && j_delta == 1 {
                        continue;
                    }

                    if i_delta != 0 && j_delta != 0 {
                        continue; // 存在对角线
                    }

                    let i_try = (i as i32 + i_delta) as usize;
                    let j_try = (j as i32 + j_delta) as usize;
                    // println!("{:?} {:?} try i j", i_try, j_try);

                    if exsits[i_try][j_try] == false {
                        if grid[i_try][j_try] == '1' {
                            // exsits[i_try][j_try] = true;

                            // 继续寻找
                            Solution::find(grid, count, m, n, exsits, i_try, j_try);
                        }
                    }
                }
            }
        }
    }
}
