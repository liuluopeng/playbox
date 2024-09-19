use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            7,
            Solution::minimum_fuel_cost(
                vec_2d_leetcode("[[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]]"),
                2
            )
        );
    }
}

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let mut graph = vec![vec![]; roads.len() + 1];

        for r in roads {
            graph[r[1] as usize].push(r[0] as usize);
            graph[r[0] as usize].push(r[1] as usize);
        }
        let seats = seats as usize;

        let mut visited = vec![false; graph.len()];

        let (sum, res) = Self::find(&graph, 0, &mut visited, seats);
        // println!("sum node: {}   oil: {}    ", sum, res);

        res as i64
    }

    fn find(
        graph: &Vec<Vec<usize>>,
        now_idx: usize,
        visited: &mut Vec<bool>,
        seats: usize,
    ) -> (usize, usize) {
        let mut counter = 0;
        let mut res = 0;
        visited[now_idx] = true;

        for &dest in &graph[now_idx] {
            if !visited[dest] {
                let (prev_counter, prev_res) = Self::find(graph, dest, visited, seats);

                counter += prev_counter;

                // println!("{}  的 prev counter:{}", dest, prev_counter);

                if prev_counter % seats == 0 {
                    res += prev_res + (prev_counter / seats);
                } else {
                    res += prev_res + (prev_counter / seats + 1);
                }
            }
        }

        (counter + 1, res)
    }
}
