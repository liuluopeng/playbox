pub fn can_topo_sort(adj: Vec<Vec<usize>>) -> bool {
    let mut in_degree = vec![0; adj.len()];
    // let mut res = 0;
    for (_node_idx, point_to) in adj.iter().enumerate() {
        for &dest in point_to {
            in_degree[dest] += 1;
        }
    }

    let mut queue_zero_in_degree = vec![];
    // 寻找入度是0的结点
    for (node_idx, &degree) in in_degree.iter().enumerate() {
        if degree == 0 {
            queue_zero_in_degree.push(node_idx);
        }
    }

    // println!("初始值 {:?} ", queue_zero_in_degree);
    let mut sort_res = vec![];

    while !queue_zero_in_degree.is_empty() {
        let node_idx = queue_zero_in_degree.remove(0);
        sort_res.push(node_idx);
        // res += 1;
        // 让所有node_idx指向的结点入度-1
        for &dest in &adj[node_idx] {
            in_degree[dest] -= 1;

            if in_degree[dest] == 0 {
                queue_zero_in_degree.push(dest);
            }
        }
    }

    // println!("最终结果  {:?}", sort_res);
    sort_res.len() == in_degree.len()
}

// [[3, 0], [3, 1], [4, 1], [4, 2], [5, 3], [5, 4]]
