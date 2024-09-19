

pub struct Solution;


impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut  res = 0;
        let n = n as usize;
        for i in 2..(n + 1){
            if Solution::is_prime(i) {
                // println!("{:?}", i);
                res += 1;
            }
        }

        
        // println!("{:?}", res);


       let resi64 : i64 =  Solution::fac(res) as i64     *  Solution::fac(n - res) as i64;
       (resi64 % 1000000007) as i32   
    }


   

    pub fn is_prime(n: usize) -> bool {
        for i in 2..n  {
            if n % i == 0 {
                return false;
            }
        }
        true 
    }

    pub fn fac(n: usize) -> usize {
        let moder = 1000000007;
        let mut res = 1;
        for i in 1..(n+1) {
            res *= i   ;
            res %= moder;
        }
        res 
    }
}




#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 5);


        assert_eq!(Solution::num_prime_arrangements(5), 12);
               assert_eq!(Solution::num_prime_arrangements(100), 682289015); 
    }
}