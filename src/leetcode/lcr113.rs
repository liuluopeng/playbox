impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut graph = vec![vec![]; num_courses];

        let mut in_deg_list = vec![0; num_courses];
        for p in prerequisites {
            // [a b]   b->a
            graph[p[1] as usize].push(p[0] as usize);
            in_deg_list[p[0] as usize] += 1;
        }

        println!("{:?}", graph);

        let mut q = vec![]; // 入度是0的队列
        for k in 0..in_deg_list.len() {
            if in_deg_list[k] == 0 {
                q.push(k);
            }
        }

        let mut res = vec![];

        while !q.is_empty() {
            let out_node_idx = q.pop().unwrap();

            res.push(out_node_idx as i32);

            for &k in &graph[out_node_idx] {
                in_deg_list[k] -= 1;
                if in_deg_list[k] == 0 {
                    q.push(k);
                }
            }
        }

        match res.len() {
            v if v == num_courses => {
                return res;
            }
            _ => {
                vec![]
            }
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::{leetcode::lcr113::Solution, util::array2d_to_vec2d};

    #[test]
    fn it_works() {
        assert_eq!(
            vec![0, 2, 1, 3],
            Solution::find_order(4, array2d_to_vec2d(&[[1, 0], [2, 0], [3, 1], [3, 2]]))
        )
    }
}
