pub struct Solution;

impl Solution {
    // 1 <= x <= y <= n
    // x + y == n
    // x 和 y 都是质数

    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        // 找质数 范围: [1 .. n]
        let prime_list = Solution::get_prime_list(n as usize);

        // println!("{:?} \n 质数表", prime_list);

        for i in 2..prime_list.len() / 2 + 1 {
            // println!("{:?} {:?}", i, n as usize - i);
            let j = n as usize - i;

            if i <= j && prime_list[i] && prime_list[j] {
                res.push(vec![i as i32, j as i32])
            }
        }

        res
    }

    pub fn get_prime_list(n: usize) -> Vec<bool> {
        // [0,1,2,3,4,5 ...n]
        let mut nums = vec![true; n + 1];
        nums[0] = false;
        nums[1] = false;
        for i in 2..nums.len() {
            let mut index = i;
            while index + i < nums.len() {
                nums[index + i] = false;
                index += i;
            }
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    // use crate::solution::Solution;

    use crate::solution::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_prime_pairs(10),
            vec![vec![3, 7], vec![5, 5],]
        );

        // let  empty_res: Vec<Vec<i32> = Vec::new(),
        // assert_eq!(
        //     Solution::find_prime_pairs(3),
        //     empty_res,
        // )
    }

    // #[test]
    // fn it_works() {
    //     let empty_res: Vec<Vec<i32> = Vec::new(),

    //     assert_eq!(
    //         Solution::find_prime_pairs(10),
    //         vec![vec![3, 7], vec![5, 5]]
    //     ),

    //     assert_eq!(
    //         Solution::find_prime_pairs(3),
    //         empty_res
    //     ),
    // }
}
