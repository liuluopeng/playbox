fn main() {
    println!("Hello, world!");

    let nums = vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8];
    let first_len = 4;
    let second_len = 3;

    println!(
        "{:?}",
        Solution::max_sum_two_no_overlap(nums, first_len, second_len)
    )
}

struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let mut res = 0;

        let mut sums = vec![0; nums.len() + 1];
        for (idx, &n) in nums.iter().enumerate() {
            sums[idx + 1] = sums[idx] + n;
        }

        println!("sums: {:?}", sums);

        let first_len = first_len as usize;
        let second_len = second_len as usize;

        let mut sumA = 0;
        for k in (first_len + second_len)..=nums.len() {
            // move or not move first_idx:
            sumA = sumA.max(sums[k - second_len] - sums[k - second_len - first_len]);

            // second_idx is moved one by one, its sum is one by one
            let part_2_sum = sums[k] - sums[k - second_len];
            res = res.max(sumA + part_2_sum);
        }
        println!("{:?} len1 len2", res);

        let mut sumB = 0;
        for k in (first_len + second_len)..=nums.len() {
            // move or not move first_idx:
            sumB = sumB.max(sums[k - first_len] - sums[k - second_len - first_len]);

            // second_idx is moved one by one, its sum is one by one
            let part_2_sum = sums[k] - sums[k - first_len];
            res = res.max(sumB + part_2_sum);
        }

        res
    }
}
