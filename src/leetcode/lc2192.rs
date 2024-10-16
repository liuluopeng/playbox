impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![]; n as usize];
        for e in edges.iter() {
            // graph[e[0] as usize].push(e[1] as usize);

            graph[e[1] as usize].push(e[0] as usize);
        }

        let mut res = vec![];
        let mut visited = vec![false; n as usize];

        for i in 0..graph.len() {
            let mut one_tuen = vec![false; graph.len()];
            let mut visit = vec![false; graph.len()];
            Self::dfs(i, &graph, &mut one_tuen, &mut visit);

            let mut one_res = vec![];
            for k in 0..one_tuen.len() {
                if one_tuen[k] == true {
                    one_res.push(k as i32);
                }
            }

            // println!("{:?} {}", one_res, i);
            res.push(one_res);
        }

        res
    }

    fn dfs(
        start: usize,
        graph: &Vec<Vec<usize>>,
        res_one_turn: &mut Vec<bool>,
        visit: &mut Vec<bool>,
    ) {
        visit[start] = true;

        for &k in &graph[start] {
            res_one_turn[k] = true;
            // visit[k] = true;

            if visit[k] == false {
                Self::dfs(k, graph, res_one_turn, visit);
            }
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::{
        leetcode::lc2192::Solution,
        util::{array2d_to_vec2d, old_vec_2d_leetcode},
    };

    #[test]
    fn it_works() {
        assert_eq!(
            old_vec_2d_leetcode("[[],[],[],[0,1],[0,2],[0,1,3],[0,1,2,3,4],[0,1,2,3]]"),
            Solution::get_ancestors(
                8,
                array2d_to_vec2d(&[
                    [0, 3],
                    [0, 4],
                    [1, 3],
                    [2, 4],
                    [2, 7],
                    [3, 5],
                    [3, 6],
                    [3, 7],
                    [4, 6]
                ])
            )
        )
    }
}
