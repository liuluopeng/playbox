use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let arr = vec![1, 3, 1, 2, 2];
    let arr = vec![
        526, 596, 12, 12, 1619, 853, 1846, 12, 232, 1722, 632, 12, 12, 596, 1124, 1866, 71, 853,
        232, 823, 50, 1545, 1268, 232, 1903, 1734, 810, 12, 1502, 1846, 325, 86, 1722, 820, 724,
        1545, 278, 476, 1268, 12, 1124, 1666, 1782, 1386, 816, 1173, 935, 1904, 1117, 678, 1582,
        632, 71, 1728, 1768, 1154, 1788, 393, 1502, 724, 1734, 557, 248, 1873, 457, 50, 940, 281,
        486, 724, 71, 685, 175, 591, 1788, 476, 1667, 348, 853, 175, 60, 1310, 280, 428, 1492,
        1722, 1932, 1386, 41, 682, 272, 1782, 1870, 1268, 557, 1058, 323, 428, 1543, 1722, 1667,
        1114, 682, 71, 71, 471, 522, 667, 1828, 398, 710, 1782, 1173, 383, 823, 632, 349, 1481,
        781, 959, 1087, 349, 113, 303, 853, 790, 1835, 1643, 46, 100, 1492, 1043, 1172, 1939, 1722,
        1834, 1643, 806, 1499, 1124, 1722, 12, 390, 1817, 1212, 818, 697, 1739, 1268, 810, 12,
        1817, 538, 374, 820, 481, 1237, 8, 98, 285, 1135, 1681, 175, 784, 232, 1221, 12, 1817,
        1873, 1189, 710, 147, 1873, 195, 311, 1728, 1624, 393, 711, 256, 1539, 476, 483, 404, 739,
        1495, 21, 810, 1667, 132, 1894, 1535, 1201, 650, 175, 541, 1513, 1822, 853, 50, 1835, 471,
        890, 689, 1461, 538, 925, 374, 1352, 1058, 907, 1268, 1377, 591, 1307, 148, 1058, 442, 953,
        509, 1198, 256, 201, 1677, 349, 442, 557, 679, 856, 545, 1062, 838, 190, 724, 303, 1441,
        1545, 1894, 959, 588, 252, 80, 1221, 700, 953, 1828, 1449, 1594, 476, 912, 1699, 1415, 825,
        1873, 632, 551, 787, 528, 713, 1135, 12, 430, 1613, 724, 1124, 1548, 22, 398, 657, 1189,
        1751, 1100, 349, 734, 532, 651, 650, 612, 710, 1681, 415, 681, 1071, 187, 968, 1641, 893,
        713, 1535, 734, 734, 372, 1634, 1548, 1059, 1793, 232, 853, 1289, 353, 1342, 202, 659,
        1818, 1793, 65, 545, 551, 1268, 663, 1299, 804, 714, 1450, 530, 1707, 1856, 1450, 856, 328,
        1692, 766, 697, 1793, 303, 1695, 1541, 1237, 1898, 133, 28, 272, 1270, 1778, 8, 1666, 512,
        787, 1542, 417, 65, 1562, 1389, 1138, 856, 1911, 914, 1271, 48, 411, 770, 318, 477, 1160,
        457, 1261, 1156, 1938, 1087, 1059, 1083, 1319, 117, 256, 1453, 383, 1017, 710, 1877, 1856,
        1230, 722, 1187, 363, 1323, 787, 160, 805, 1677, 1974, 1270, 63, 19, 476, 1450, 1055, 775,
        1216, 695, 543, 1954, 1221, 810, 1529, 1186, 1731, 784, 522, 1029, 1924, 1503, 1204, 1309,
        1845, 31, 630, 1956, 1903, 1309, 856, 1407, 226, 1642, 303, 1822, 1697, 1479, 1296, 718,
        541, 1994, 1775, 968, 304, 1582, 989, 437, 270, 1582, 539, 1610, 951, 1425, 1343, 78, 181,
        1497, 722, 1393, 1604, 281, 1386, 148, 417, 1741, 657, 1466, 214, 1575, 175, 724, 1094,
        1458, 1469, 126, 133, 578, 1882, 1710, 423, 853, 1347, 1042,
    ];
    println!("{:?}", Solution::count_complete_subarrays(arr));
}

struct Solution;

/*
输入：nums = [1,3,1,2,2]
输出：4
解释：完全子数组有：[1,3,1,2]、[1,3,1,2,2]、[3,1,2] 和 [3,1,2,2] 。


*/

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut counter = 0;

        let mut sett = HashMap::new();
        for i in nums.iter() {
            *sett.entry(i).or_insert(0) += 1;
        }

        let mut res = vec![];

        let mut m = HashMap::new();

        for i in 0..nums.len() {
            m.clear();
            let mut line = vec![];

            for j in i + 1..nums.len() {
                
                if  m.len() == 

                for idx in i..=j {
                    *m.entry(nums[idx]).or_insert(0) += 1;
                }

                line.push(m.len());
            }

            res.push(line);
        }

        for i in res {
            for j in i {
                if j == sett.len() {
                    counter += 1;
                }
            }
        }

        counter
    }
}
