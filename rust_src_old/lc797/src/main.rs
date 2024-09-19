struct Solution;

impl Solution {
    // graph: [[1,2],[3],[3],[]] 邻接表
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let mut target = graph.len() as i32 - 1;

        Self::find(&graph, &mut res, &mut vec![], 0, target);

        res
    }

    pub fn find(
        graph: &Vec<Vec<i32>>,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        start: i32,
        target: i32,
    ) {
        // println!("{:?}", path);

        let mut path = path.clone();
        path.push(start);
        if path[path.len() - 1] == target {
            res.push(path.to_vec());
        }

        let maybe = &graph[start as usize];
        for choice in maybe {
            // println!("{:?}", choice);
            Self::find(graph, res, &mut path, *choice, target);
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let mut graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];

    println!("{:?}", Solution::all_paths_source_target(graph));
}
