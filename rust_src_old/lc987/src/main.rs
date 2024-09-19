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

    // for n in &tree_node_list {
    //     println!("节点打印 {:?}", n);
    // }

    let head = &tree_node_list[0];

    let mut queue = vec![tree_node_list[0].clone()];
    let mut front = 0;
    let mut index = 1;

    while index < node_list.len() {
        let node = queue[front].clone();
        front += 1;

        // println!("当前节点 {:?}  现在的index {:?}", node.borrow().val, index);

        let str_item = node_list[index].clone();
        if str_item != "null".to_string() {
            // println!("构建关系 节点 {:?} 左子树 {:?}", node.borrow().val, tree_node_list[index].borrow().val);
            node.borrow_mut().left = Some(tree_node_list[index].clone());
            queue.push(tree_node_list[index].clone());
        }
        index += 1;

        if index == node_list.len() {
            break;
        }

        let str_item = node_list[index].clone();
        if str_item != "null".to_string() {
            // println!("构建关系 节点 {:?} 右子树 {:?}", node.borrow().val, tree_node_list[index].borrow().val);

            node.borrow_mut().right = Some(tree_node_list[index].clone());
            queue.push(tree_node_list[index].clone());
        }
        index += 1;
    }

    Some(head.clone())
}

fn main() {
    println!("Hello, world!");

    let st = "[3,9,20,null,null,15,7]";
    let root = string2tree(st);
    let res = Solution::vertical_traversal(root);
    
}

struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        // 先找出每个节点 的 深度、相对中垂线的位置。
        let mut depth_msg = vec![];
        let mut offset_msg = vec![];
        let mut node_val_list = vec![];


        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            my_depth: i32,
            my_offset: i32,
            depth_msg: &mut Vec<i32>,
            offset_msg: &mut Vec<i32>,
            node_val_list: &mut Vec<i32>,
        ) {
            if let Some(node) = root {
                // 向三个数组添加元素
                node_val_list.push(node.borrow().val);
                depth_msg.push(my_depth);
                offset_msg.push(my_offset);

                if node.borrow().left.is_some() {
                    dfs(
                        node.borrow().left.clone(),
                        my_depth + 1,
                        my_offset - 1,
                        depth_msg,
                        offset_msg,
                        node_val_list,
                    );
                }
                if node.borrow().right.is_some() {
                    dfs(
                        node.borrow().right.clone(),
                        my_depth + 1,
                        my_offset + 1,
                        depth_msg,
                        offset_msg,
                        node_val_list,
                    );
                }
            }
        }

        dfs(
            root,
            0,
            0,
            &mut depth_msg,
            &mut offset_msg,
            &mut node_val_list,
        );

        // 按照offset分组
        let mut offset_set: HashSet<i32> = HashSet::new();
        let mut depth_set: HashSet<i32> = HashSet::new();
        for i in offset_msg.iter() {
            offset_set.insert(*i);
        }
        for i in depth_msg.iter(){
            depth_set.insert(*i);
        }


        // 将 HashSet 转换为 Vec，并排序
        let mut offset_range: Vec<i32> = offset_set.into_iter().collect();
        offset_range.sort();

      
        let mut depth_range: Vec<i32> = depth_set.into_iter().collect();
        depth_range.sort();


        for offset in offset_range.iter() {
            let mut a_res = vec![];
            for depth in depth_range.iter() {
                let mut cadi = vec![];
                for i in 0..node_val_list.len() {
                    if offset_msg[i]==*offset && depth_msg[i] == *depth{
                        cadi.push(node_val_list[i])
                    }
                }
                cadi.sort();
                a_res.extend(cadi);
            }


            res.push(a_res);
        }


        res
    }
}
