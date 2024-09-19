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

fn deserialize(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
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

struct Solution;

impl Solution {
    pub fn get_right_width(arr: &Vec<char>) -> i32 {
        let mut res = 0;
        let length = arr.len();

        let mut time = 1;

        let mut left_bound = 1;
        let mut right_bound = 2_i32.pow((length - time).try_into().unwrap());

        while time < length && left_bound < right_bound {
            if arr[time] == 'R' {
                left_bound = left_bound + (right_bound - left_bound) / 2;
            } else {
                // arr[time] == 'L'
                right_bound = right_bound - (right_bound - left_bound) / 2;
            }
            time += 1;
        }
        // println!("{} {}right ", left_bound, right_bound);

        right_bound
    }

    pub fn get_left_width(arr: &Vec<char>) -> i32 {
        let mut res = 0;
        let length = arr.len();

        let mut time = 1;

        let mut left_bound = 2_i32.pow((length - time).try_into().unwrap());
        let mut right_bound = 1;

        while time < length && left_bound < right_bound {
            if arr[time] == 'L' {
                right_bound = right_bound - (right_bound - left_bound) / 2;
            } else {
                // arr[time] == 'R'
                left_bound = left_bound + (right_bound - left_bound) / 2;
            }
            time += 1;
        }

        // println!("{} {}left", left_bound, right_bound);
        left_bound
    }

    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = 0;

        let mut left_side_path = vec![];
        let mut right_side_path = vec![];

        if let Some(root) = root {
            let mut current_left = root.borrow().left.clone();
            let mut current_right = root.borrow().right.clone();

            if current_left.is_some() && current_right.is_some() {
                left_side_path.push('L');
                right_side_path.push('R');
                max_sum = 2;
            }

            while let (Some(node_left), Some(node_right)) =
                (current_left.clone(), current_right.clone())
            {
                let node_left_ref = node_left.borrow();
                let node_right_ref = node_right.borrow();

                if (node_left_ref.left.is_some() || node_left_ref.right.is_some())
                    && (node_right_ref.left.is_some() || node_right_ref.right.is_some())
                // 两侧节点都存在下一层
                {
                    if node_left_ref.left.is_some() {
                        let left_of_left = &node_left_ref.left.clone().unwrap();
                        current_left = Some(Rc::clone(left_of_left));

                        // left_side_width += 1;
                        left_side_path.push('L');
                    } else {
                        // 那么只能选右侧
                        let right_of_left = &node_left_ref.right.clone().unwrap();
                        current_left = Some(Rc::clone(right_of_left));

                        // left_side_width -= 1;
                        left_side_path.push('R');
                    }

                    if node_right_ref.right.is_some() {
                        let right_of_right = &node_right_ref.right.clone().unwrap();
                        current_right = Some(right_of_right.clone());
                        // right_side_width += 1;

                        right_side_path.push('R');
                    } else {
                        // 那么只能选左侧
                        current_right = node_right_ref.left.clone();
                        // right_side_width -= 1;

                        right_side_path.push('L');
                    }

                    // 一层结束，统计总和。 并保持总和始终是记录到的最大的。

                    let r_res = Self::get_right_width(&right_side_path);

                    let l_res = Self::get_left_width(&left_side_path);
                    println!(
                        " currtne left {:?} {:?}   current right : {:?} {:?} ",
                        left_side_path, l_res, right_side_path, r_res
                    );
                    if l_res + r_res > max_sum {
                        max_sum = l_res + r_res;
                    }
                } else {
                    // 没有下一层了
                    break;
                }
            }
        }

        // left_side_width + right_side_width
        max_sum
    }
}

fn main() {
    let st = "[1,3,2,5,3,null,9]";
    let st = "[1,3,2,5,null,null,9,6,null,7]";
    let st = "[1,3,2,5]";


    let root = deserialize(st);

    let res = Solution::width_of_binary_tree(root);

    println!("{}", res);
}
