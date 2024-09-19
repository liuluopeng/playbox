// 各种工具函数

// 在rust中，给二维数组每行增加一个“vec”
pub fn vec_2d_leetcode(input: &str) -> Vec<Vec<i32>> {
    let input = input.trim_matches(|c| c == '[' || c == ']' || c == ',');

    let mut result: Vec<Vec<i32>> = Vec::new();
    for inner_vec_str in input.split("],[").map(|s| s.split(',').collect::<Vec<_>>()) {
        let inner_vec: Vec<i32> = inner_vec_str
            .iter()
            .filter_map(|&x| x.parse().ok())
            .collect();
        result.push(inner_vec);
    }
    result
}
// [[2,1,1],[2,3,1],[3,4,1]]   2指向1,权重1
// start: 1
// pub fn from_leetcode(raw: Vec<Vec<i32>>, size: usize) -> Vec<Vec<(usize, i32)>> {
//     let mut data = vec![vec![]; size];

//     for edge in raw {
//         data[edge[0] as usize - 1].push((edge[1] as usize - 1, edge[2]));
//     }
// }




// ["1","3","5","7"] => Vec<String>
pub fn vec_of_string()  {

    let s = vec!["1","3","5","7"];

    let ss: Vec<String> = s.iter().map(|x| x.to_string()).collect();

    

}