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

// 打印二叉树
pub fn print(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        println!(
            "节点 {:?}  左孩子 {:?}  右孩子 {:?}",
            &node.borrow().val,
            &node.borrow().left,
            &node.borrow().right
        );

        print(&node.borrow().left);
        print(&node.borrow().right);
    }
}

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
fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
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

fn deserialize(data: String) -> Option<Rc<RefCell<TreeNode>>> {
    let t_str = data.as_str();
    let mut t_sstr = t_str.to_string();
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

struct Solution;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val == val {
                return Some(node);
            }

            if node.borrow().val < val {
                return Self::search_bst(node.borrow().right.clone(), val);
            }

            if node.borrow().val > val {
                return Self::search_bst(node.borrow().left.clone(), val);
            }
        }

        return None;
    }
}

fn main() {
    println!("Hello, world!");

    let st = "[4,2,7,1,3]".to_string();
    let root = deserialize(st);
    let stt = serialize(root.clone());
    println!("{}", stt);

    println!("{:?}", Solution::search_bst(root, 33));
}
