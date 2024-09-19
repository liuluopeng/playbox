fn main() {
    println!("Hello, world!");

    let st = "[[2],[3,4],[6,5,7],[4,1,8,3]]";

    let arr2d = conv(st);

    println!("{:?}", Solution::minimum_total(arr2d));
}

// 在rust中，给二维数组每行增加一个“vec”
fn conv(input: &str) -> Vec<Vec<i32>> {
    let input = input.trim_matches(|c| c == '[' || c == ']' || c == ',');

    let mut result: Vec<Vec<i32>> = Vec::new();
    for inner_vec_str in input.split("],[").map(|s| s.split(',').collect::<Vec<_>>()) {
        let inner_vec: Vec<i32> = inner_vec_str
            .iter()
            .filter_map(|&x| x.parse().ok())
            .collect();
        result.push(inner_vec);
    }
    result
}
struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut memo2d: Vec<Vec<i32>> = vec![];

        for i in 0..triangle.len() {
            let mut memo1d = vec![];

            for j in 0..triangle[i].len() {
                let mut less = 0;
                if i == 0 {
                    less = 0;
                } else if j == 0 {
                    less = memo2d[i - 1][0];
                } else if j == triangle[i].len() - 1 {
                    less = memo2d[i - 1][triangle[i - 1].len() - 1];
                } else {
                    less = memo2d[i - 1][j].min(memo2d[i - 1][j - 1]);
                }

                let one_res = triangle[i][j] + less;

                // println!("一个结果:  {:?} {:?} {:?}", i, j, one_res);

                memo1d.push(one_res);
            }

            memo2d.push(memo1d);
        }

        let mut res: i32 = memo2d[memo2d.len() - 1][0];
        for i in 0..memo2d[memo2d.len() - 1].len() {
            res = res.min(memo2d[memo2d.len() - 1][i]);
            // println!("{}", res);
        }
        res
    }
}
