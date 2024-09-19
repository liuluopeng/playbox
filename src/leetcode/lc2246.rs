use crate::solution::Solution;

#[cfg(test)]
mod tests {
    use crate::{solution::Solution, util::vec_2d_leetcode};

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], String::from("abacbe"))
        );

        assert_eq!(17, Solution::longest_path(vec![-1,137,65,60,73,138,81,17,45,163,145,99,29,162,19,20,132,132,13,60,21,18,155,65,13,163,125,102,96,60,50,101,100,86,162,42,162,94,21,56,45,56,13,23,101,76,57,89,4,161,16,139,29,60,44,127,19,68,71,55,13,36,148,129,75,41,107,91,52,42,93,85,125,89,132,13,141,21,152,21,79,160,130,103,46,65,71,33,129,0,19,148,65,125,41,38,104,115,130,164,138,108,65,31,13,60,29,116,26,58,118,10,138,14,28,91,60,47,2,149,99,28,154,71,96,60,106,79,129,83,42,102,34,41,55,31,154,26,34,127,42,133,113,125,113,13,54,132,13,56,13,42,102,135,130,75,25,80,159,39,29,41,89,85,19], String::from("ajunvefrdrpgxltugqqrwisyfwwtldxjgaxsbbkhvuqeoigqssefoyngykgtthpzvsxgxrqedntvsjcpdnupvqtroxmbpsdwoswxfarnixkvcimzgvrevxnxtkkovwxcjmtgqrrsqyshxbfxptuvqrytctujnzzydhpal")));
    }
}

struct Info {
    saved_res: usize,   // 保存的答案
    res_from_me: usize, // 从我开始往下的最大答案
}

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut graph = vec![vec![]; parent.len()];

        for i in 1..parent.len() {
            graph[parent[i] as usize].push(i)
        }

        let mut ss: Vec<char> = s.chars().collect();

        let info = Self::find(&graph, &ss, 0, None);
        info.saved_res as i32
    }

    fn find(graph: &Vec<Vec<usize>>, ss: &Vec<char>, me: usize, my_father: Option<usize>) -> Info {
        let mut info = Info {
            saved_res: 1,
            res_from_me: 1,
        };

        let my_char = ss[me];

        let mut morest = 0;
        let mut more = 0;

        for &dest in &graph[me] {
            let prev_res = Self::find(graph, ss, dest, my_father);
            let child_char = ss[dest];

            info.saved_res = info.saved_res.max(prev_res.saved_res);

            if my_char == child_char {
                // 只能取得各个孩子的最大长度
                // info.res_throgh_me =
            } else {
                // my_chr != child_char,可以延长

                // if prev_res.res_throgh_me > more {
                //     more = prev_res.res_throgh_me;

                //     if more > morest {
                //         morest = more;

                //     }
                // }

                if prev_res.res_from_me > morest {
                    more = morest; // 提前保存,
                    morest = prev_res.res_from_me;
                } else if prev_res.res_from_me > more {
                    more = prev_res.res_from_me;
                }
            }
        }

        info.res_from_me = 1 + morest;
        info.saved_res = info.saved_res.max(1 + more + morest);

        info
    }
}
