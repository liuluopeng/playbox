use std::collections::HashMap;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;
        let mut graph = vec![vec![]; n];
        let mut indgree_list = vec![0; n];
        for i in 0..prerequisites.len() {
            graph[prerequisites[i][0] as usize].push(prerequisites[i][1] as usize);
            indgree_list[prerequisites[i][1] as usize] += 1;
        }

        let mut queue = vec![];
        for i in 0..indgree_list.len() {
            if indgree_list[i] == 0 {
                queue.push(i);
            }
        }

        let mut a_can_go_b = vec![vec![false; n]; n];

        while !queue.is_empty() {
            let start = queue.pop().unwrap();

            for &k in &graph[start] {
                a_can_go_b[start][k] = true;

                for ii in 0..n {
                    a_can_go_b[ii][k] = a_can_go_b[ii][k] | a_can_go_b[ii][start];
                }

                indgree_list[k] -= 1;
                if indgree_list[k] == 0 {
                    queue.push(k);
                }
            }
        }

        let mut res = vec![];

        for q in queries {
            let start = q[0] as usize;
            let end = q[1] as usize;

            res.push(a_can_go_b[start][end]);
        }
        res
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
