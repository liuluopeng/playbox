struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let m: usize = nums1.len();
        let n: usize = nums2.len();

        let mut res: f64 = 0.0;

        let mut aux_nums: Vec<i32> = vec![];

        while i < m || j < n {
            println!("{:?} {:?}", i, j);
            if i == m {
                aux_nums.push(nums2[j]);
                j += 1;
            } else if j == n {
                aux_nums.push(nums1[i]);
                i += 1;
            } else if nums1[i] < nums2[j] {
                aux_nums.push(nums1[i]);
                i += 1;
            } else {
                aux_nums.push(nums2[j]);
                j += 1;
            }
        }

        println!("{:?}", aux_nums);

        res = ((aux_nums[(m + n - 1) / 2]) as f64 + (aux_nums[(m + n) / 2]) as f64) / 2.0;

        res
    }
}

fn main() {
    let nums1 = vec![0, 0];

    let nums2 = vec![0, 0];

    println!("{}", Solution::find_median_sorted_arrays(nums1, nums2));
}
