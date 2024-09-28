use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

// 定义一个结构体来存储你的元素
#[derive(Debug, Eq, PartialEq)]
struct MyElement {
    value: i32,
    name: String,
}
// 实现 Ord 和 PartialOrd 来定义排序规则
impl Ord for MyElement {
    fn cmp(&self, other: &Self) -> Ordering {
        // 首先按照 i32 的值排序
        self.value
            .cmp(&other.value)
            // 如果 i32 的值相同，则按照 String 排序
            .then_with(|| other.name.cmp(&self.name))
    }
}

// 实现 PartialOrd 来使用 Ord 的实现
impl PartialOrd for MyElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// impl PartialEq for WordFrequency {
//     fn eq(&self, other: &Self) -> bool {
//         self.frequency == other.frequency && self.word == other.word
//     }
// }

// 作者：Ganlv
// 链接：https://leetcode.cn/problems/top-k-frequent-words/solutions/786205/rust-hashmap-ji-shu-binaryheap-da-ding-d-eui5/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut res = vec![];

        let mut queue = BinaryHeap::new();

        let mut cnt_map = HashMap::new();

        for word in words {
            let time = cnt_map.entry(word).or_insert(0);
            *time += 1;
        }

        for (k, v) in cnt_map {
            // queue.push((v, k));
            queue.push(MyElement { value: v, name: k });
        }

        for i in 0..k as usize {
            res.push(queue.pop().unwrap().name)
        }

        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::lc692::Solution;

    #[test]
    fn it_works() {
        let s: Vec<String> = ["i", "love"]
            .to_vec()
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            s,
            Solution::top_k_frequent(
                ["i", "love", "leetcode", "i", "love", "coding"]
                    .to_vec()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                2
            )
        );
    }
}
