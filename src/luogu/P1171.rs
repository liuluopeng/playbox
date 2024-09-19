use std::{
    io::{self, *},
    usize,
};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取第一行
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = parts[0];

    let mut graph: Vec<Vec<u16>> = vec![];

    // 读取接下来的 M 行
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<u16> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        graph.push(parts);
    }

    println!("{:?}", graph);

    let res = slove(n, graph);
    println!("{:?}", res);
}
fn slove(n: usize, graph: Vec<Vec<u16>>) -> u16 {
    //  选取去到的村子, 还有从哪个村子出发   的  耗费
    let mut record: Vec<Vec<u16>> = vec![vec![0; 1 << n]; graph.len()];

    let mut setted: Vec<Vec<bool>> = vec![vec![false; 1 << n]; graph.len()];

    // 1: 站在0_idx的村庄出发. 0_idx已经被访问
    let res = find(&mut record, &mut setted, 1, 0, &graph);

    return res;
}

// 从某一种情况出发,完成访问所有村落的路程.  这种情况之前的路程不关心.
fn find(
    record: &mut Vec<Vec<u16>>,
    setted: &mut Vec<Vec<bool>>,
    situation_idx: usize,
    from_idx: usize,
    graph: &Vec<Vec<u16>>,
) -> u16 {
    if setted[from_idx][situation_idx] == true {
        return record[from_idx][situation_idx];
    }

    if situation_idx == (1 << graph.len()) - 1 {
        record[from_idx][situation_idx] = graph[from_idx][0];
        return graph[from_idx][0];
    }

    let mut curr_min_dist = u16::MAX;

    for k_idx in 0..graph[from_idx].len() {
        if situation_idx & (1 << k_idx) == 0 {
            let this_try = find(record, setted, situation_idx | (1 << k_idx), k_idx, graph);

            curr_min_dist = curr_min_dist.min(graph[from_idx][k_idx] + this_try);
        }
    }

    record[from_idx][situation_idx] = curr_min_dist;
    setted[from_idx][situation_idx] = true;
    return curr_min_dist;
}

// 为什么多添加了路程?
fn find_llp(
    record: &mut Vec<Vec<usize>>,
    situation_idx: usize,
    from_idx: usize,
    graph: &Vec<Vec<usize>>,
    accu_dist: usize, // 当前尝试中, 已经走过的路程
    accu_detail: Vec<(usize, usize)>,
    final_res: &mut Vec<usize>,
) -> usize {
    // if record[situation_idx][from_idx] != 0 {
    //     return record[situation_idx][from_idx];
    // }

    if situation_idx == (1 << graph.len()) - 1 {
        // if record[situation_idx][from_idx] != 0 {
        //     println!(
        //         "企图第二次复制 {}   {}  {}",
        //         situation_idx, from_idx, record[situation_idx][from_idx]
        //     );
        // }

        if record[situation_idx][from_idx] == 0 {
            record[situation_idx][from_idx] = graph[from_idx][0] + accu_dist;
        } else {
            let ori = record[situation_idx][from_idx];

            record[situation_idx][from_idx] = ori.max(accu_dist + graph[from_idx][0]);
        }

        final_res.push(record[situation_idx][from_idx]);

        println!("路程到底哪里重复计算了: {:?}", &accu_detail);

        let mut rep_sum = 0;
        for i in accu_detail.clone() {
            rep_sum += graph[i.0][i.1];
        }
        println!("手算的是  {}", rep_sum);

        // return accu_dist;
        // return graph[from_idx][0] + accu_dist;
    }

    if record[situation_idx][from_idx] != 0 {
        return record[situation_idx][from_idx];
    } else {
        let mut curr_min_dist = usize::MAX;

        for k_idx in 0..graph[from_idx].len() {
            if situation_idx & (1 << k_idx) == 0 {
                let new_from_to = (from_idx, k_idx);
                let mut new_accu_dtail = accu_detail.clone();
                new_accu_dtail.push(new_from_to);

                let this_try = find_llp(
                    record,
                    situation_idx | (1 << k_idx),
                    k_idx,
                    graph,
                    accu_dist + graph[from_idx][k_idx],
                    new_accu_dtail,
                    final_res,
                );

                curr_min_dist = curr_min_dist.min(this_try);
            }
        }
        record[situation_idx][from_idx] = curr_min_dist;
        return record[situation_idx][from_idx];
    }
}

// 暴力算法: 除了起点之外, 进行全排列.
fn brute(
    graph: &Vec<Vec<usize>>,
    path: &mut Vec<usize>,
    accu: &mut usize,
    st_idx: usize,
    shorest: &mut usize,
) {
    if path.len() == graph.len() - 1 {
        let sum: usize = *accu;

        let sum = sum + graph[path[path.len() - 1]][0];

        // println!("{:?} path", path);

        if sum < *shorest {
            *shorest = sum;
        }
    }

    for k in 1..graph.len() {
        let mut last_idx = 33;

        if path.len() == 0 {
            last_idx = 0;
        } else {
            last_idx = path[path.len() - 1];
        }

        let mut contail = false;
        for kk in 0..path.len() {
            if path[kk] == k {
                contail = true;
                break;
            }
        }
        if !contail {
            path.push(k);
            *accu += graph[last_idx][k];

            brute(graph, path, accu, k + 1, shorest);

            path.pop();
            *accu -= graph[last_idx][k];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{main, slove};

    #[test]
    fn it_works() {
        let a = [
            [0, 487, 975, 636, 836, 688, 546, 411, 748, 199],
            [521, 0, 203, 268, 533, 102, 906, 424, 871, 173],
            [567, 758, 0, 612, 128, 509, 206, 3, 417, 554],
            [164, 848, 953, 0, 95, 696, 825, 962, 360, 933],
            [105, 360, 246, 232, 0, 513, 498, 482, 2, 643],
            [106, 901, 679, 994, 628, 0, 932, 116, 993, 260],
            [186, 412, 397, 611, 822, 386, 0, 144, 720, 807],
            [20, 437, 896, 787, 717, 851, 439, 0, 20, 519],
            [253, 747, 384, 282, 353, 245, 323, 261, 0, 611],
            [65, 550, 816, 327, 259, 925, 245, 624, 116, 0],
        ];
        let a = [
            [
                0, 487, 975, 636, 836, 688, 546, 411, 748, 199, 302, 808, 945, 34, 631, 462, 644,
                551, 401, 128,
            ],
            [
                521, 0, 203, 268, 533, 102, 906, 424, 871, 173, 256, 450, 7, 283, 80, 266, 315,
                451, 327, 960,
            ],
            [
                567, 758, 0, 612, 128, 509, 206, 3, 417, 554, 576, 776, 293, 857, 928, 998, 583,
                44, 25, 274,
            ],
            [
                164, 848, 953, 0, 95, 696, 825, 962, 360, 933, 390, 159, 20, 76, 923, 29, 110, 323,
                742, 904,
            ],
            [
                105, 360, 246, 232, 0, 513, 498, 482, 2, 643, 276, 333, 83, 375, 244, 711, 778,
                226, 190, 925,
            ],
            [
                106, 901, 679, 994, 628, 0, 932, 116, 993, 260, 224, 654, 387, 150, 984, 61, 135,
                571, 574, 696,
            ],
            [
                186, 412, 397, 611, 822, 386, 0, 144, 720, 807, 840, 66, 99, 794, 862, 466, 195,
                510, 570, 758,
            ],
            [
                20, 437, 896, 787, 717, 851, 439, 0, 20, 519, 683, 363, 672, 887, 702, 191, 861,
                276, 656, 460,
            ],
            [
                253, 747, 384, 282, 353, 245, 323, 261, 0, 611, 943, 977, 336, 751, 805, 397, 645,
                697, 354, 210,
            ],
            [
                65, 550, 816, 327, 259, 925, 245, 624, 116, 0, 16, 947, 278, 975, 864, 881, 903,
                995, 988, 335,
            ],
            [
                59, 879, 802, 26, 177, 956, 931, 136, 647, 1000, 0, 979, 109, 176, 170, 136, 285,
                334, 247, 380,
            ],
            [
                79, 509, 30, 199, 89, 565, 995, 523, 996, 474, 639, 0, 913, 426, 868, 780, 478,
                961, 818, 771,
            ],
            [
                25, 261, 283, 977, 188, 321, 933, 444, 495, 404, 141, 731, 0, 359, 754, 160, 873,
                675, 23, 670,
            ],
            [
                861, 397, 846, 5, 836, 590, 732, 87, 404, 694, 733, 306, 419, 0, 558, 735, 426,
                583, 900, 803,
            ],
            [
                107, 195, 711, 172, 677, 417, 668, 887, 403, 338, 729, 580, 929, 374, 0, 320, 35,
                440, 697, 926,
            ],
            [
                344, 339, 556, 54, 926, 787, 231, 942, 295, 929, 292, 132, 918, 927, 836, 0, 613,
                375, 339, 77,
            ],
            [
                161, 686, 640, 980, 99, 713, 607, 460, 116, 937, 333, 133, 23, 717, 445, 254, 0,
                227, 635, 486,
            ],
            [
                383, 349, 942, 16, 636, 911, 667, 138, 420, 558, 805, 933, 729, 708, 422, 470, 222,
                0, 491, 786,
            ],
            [
                373, 51, 811, 751, 602, 41, 899, 365, 786, 411, 72, 475, 495, 788, 587, 919, 241,
                340, 0, 587,
            ],
            [
                903, 629, 674, 219, 265, 966, 890, 228, 665, 176, 515, 52, 565, 3, 530, 490, 431,
                357, 173, 0,
            ],
        ];
        // 将二维数组转换为 Vec<Vec<usize>>
        let graph: Vec<Vec<u16>> = a
            .iter()
            .map(|row| row.iter().map(|&x| x as u16).collect())
            .collect();

        slove(graph.len(), graph);

        // main()
    }
}
