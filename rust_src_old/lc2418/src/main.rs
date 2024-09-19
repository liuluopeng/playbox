use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut res = vec![];

        let mut hm = HashMap::new();

        for i in 0..names.len() {
            hm.insert(heights[i], names[i].clone());
        }

        let n = heights.len();
        let mut heights = heights.clone();
        Self::quick_sort(&mut heights, 0, n - 1);

        println!("{:?}", heights);

        for i in heights {
            res.push(hm.get(&i).unwrap().to_string());
        }

        res
    }

    pub fn quick_sort(arr: &mut Vec<i32>, l: usize, r: usize) {
        if l >= r {
            return;
        }

        let povit = &arr[r].clone();

        let mut wait_index = l;

        for i in l..r {
            if arr[i] > *povit {
                arr.swap(i, wait_index);
                wait_index += 1;
            }
        }

        arr.swap(wait_index, r);

        if wait_index > 0 {
            Self::quick_sort(arr, l, wait_index - 1);
            Self::quick_sort(arr, wait_index + 1, r);
        } else {
            Self::quick_sort(arr, wait_index + 1, r);
        }
    }
}

fn main() {
    let names = vec![
        String::from("Mary"),
        String::from("Marys"),
        String::from("Mary1"),
        String::from("Mary23"),
    ];

    let heights = vec![180, 165, 170, -2];

    println!("{:?}", Solution::sort_people(names, heights))
}
