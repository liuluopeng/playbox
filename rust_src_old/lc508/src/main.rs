// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn count_node(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut counter = 0;

    fn find(root: &Option<Rc<RefCell<TreeNode>>>, counter: &mut i32) {
        match root {
            None => {}
            Some(node) => {
                let node = node.borrow();
                // println!("访问的节点是 {:?}", node.val);
                *counter += 1;

                find(&node.left, counter);
                find(&node.right, counter);
            }
        }
    }

    find(root, &mut counter);
    counter
}
// 把[1,None,2]变成字符串
pub fn nodelist2string(nodelist: Vec<Option<i32>>) -> String {
    let mut res = String::from("[");

    for i in 0..nodelist.len() {
        match nodelist[i] {
            None => res.push_str("null"),
            Some(val) => {
                res += &val.to_string();
            }
        }
        if i != nodelist.len() - 1 {
            res.push(',');
        }
    }

    res.push(']');
    res
}
fn tree2string(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut node_list: Vec<Option<i32>> = vec![];
    let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

    let mut last_index = count_node(&root); // 最后一个叶子节点的序号

    match root {
        None => {}
        Some(node) => {
            queue.push(Some(node));
        }
    }

    let mut index = 0;

    while !queue.is_empty() {
        let node = queue.remove(0);
        // println!("index: {:?}", index);
        match node {
            None => {
                node_list.push(None);
            }
            Some(node) => {
                node_list.push(Some(node.borrow().val));
                index += 1;

                queue.push(node.borrow().left.clone());
                queue.push(node.borrow().right.clone());

                if index == last_index {
                    break;
                }
            }
        }
    }

    nodelist2string(node_list)
}

fn string2tree(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let mut t_sstr = data.to_string();
    t_sstr = t_sstr[1..t_sstr.len() - 1].to_string();

    if t_sstr.len() == 0 {
        return None;
    }

    let node_list: Vec<String> = t_sstr.split(",").map(|s| s.to_string()).collect();

    //  可以算出深度  可以算出  父节点是谁。
    let mut tree_node_list = vec![];
    // 创建节点
    for (index, st) in node_list.iter().enumerate() {
        if *st != String::from("null") {
            let node = Rc::new(RefCell::new(TreeNode::new(st.parse().unwrap())));
            tree_node_list.push(node);
        } else {
            // 空的， 先放一些数字的0的节点占住index。
            let node = Rc::new(RefCell::new(TreeNode::new(550)));
            tree_node_list.push(node);
        }
    }

    let head = &tree_node_list[0];

    let mut queue = vec![tree_node_list[0].clone()];
    let mut front = 0;
    let mut index = 1;

    while index < node_list.len() {
        let node = queue[front].clone();
        front += 1;

        let str_item = node_list[index].clone();
        if str_item != "null".to_string() {
            node.borrow_mut().left = Some(tree_node_list[index].clone());
            queue.push(tree_node_list[index].clone());
        }
        index += 1;

        if index == node_list.len() {
            break;
        }

        let str_item = node_list[index].clone();
        if str_item != "null".to_string() {
            node.borrow_mut().right = Some(tree_node_list[index].clone());
            queue.push(tree_node_list[index].clone());
        }
        index += 1;
    }

    Some(head.clone())
}

fn main() {
    let st = "[5,2,1]";
    let st = "[5,2,-3]";
    let st = "[5,2,-5]";
    let root = string2tree(st);
    let res = Solution::find_frequent_tree_sum(root);
    println!("{:?}", res);
}

struct Solution;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut cadi_map = HashMap::new(); // key:节点之和  value:出现次数
        let mut max_time = -1;

        Self::find(root, &mut cadi_map, &mut max_time);

        for (k, v) in cadi_map.iter() {
            if *v == max_time {
                res.push(*k);
            }
        }

        return res
    }

    pub fn find(
        node: Option<Rc<RefCell<TreeNode>>>,
        cadi_map: &mut HashMap<i32, i32>,
        max_time: &mut i32,
    ) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();

            let left_sum = Self::find(node.left.clone(), cadi_map, max_time);
            let right_sum = Self::find(node.right.clone(), cadi_map, max_time);

            let sum_of_node = node.val + left_sum + right_sum;
            let count = cadi_map.entry(sum_of_node).or_insert(0);
            *count += 1;

            if *count > *max_time {
                *max_time = *count;
            }

            return sum_of_node;
        } else {
            0
        }
    }
}
