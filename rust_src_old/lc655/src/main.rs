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
    let st: &str = "[1,2,3,null,4]";

    let root = string2tree(st);

    Solution::print_tree(root);
}

struct Solution {}

/*

给你一棵二叉树的根节点 root ，请你构造一个下标从 0 开始、大小为 m x n 的字符串矩阵 res ，用以表示树的 格式化布局 。构造此格式化布局矩阵需要遵循以下规则：

树的 高度 为 height ，矩阵的行数 m 应该等于 height + 1 。
矩阵的列数 n 应该等于 2height+1 - 1 。
根节点 需要放置在 顶行 的 正中间 ，对应位置为 res[0][(n-1)/2] 。
对于放置在矩阵中的每个节点，设对应位置为 res[r][c] ，将其左子节点放置在 res[r+1][c-2height-r-1] ，右子节点放置在 res[r+1][c+2height-r-1] 。
继续这一过程，直到树中的所有节点都妥善放置。
任意空单元格都应该包含空字符串 "" 。
返回构造得到的矩阵 res 。
*/

impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let mut res = vec![];

        fn get_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root {
                let left_depth = get_height(node.borrow().left.clone());
                let right_depth = get_height(node.borrow().right.clone());
                return 1 + left_depth.max(right_depth);
            }
            0
        }

        let height = get_height(root.clone()) - 1;

        // 行数
        let m = height + 1;
        // 列数
        let n = 2i32.pow((height + 1).try_into().unwrap()) - 1;

        for i in 0..m {
            let row = vec![String::from(""); n as usize];
            res.push(row);
        }

        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            res: &mut Vec<Vec<String>>,
            i: usize,
            j: usize,
            height: i32,
        ) {
            if let Some(node) = root {
                let node = node.borrow();
                res[i][j] = node.val.to_string();

                if node.left.is_some() {
                    let li = i + 1;
                    let lj = j - 2usize.pow(height as u32 - i as u32 - 1);
                    dfs(&node.left, res, li, lj, height);
                }

                if node.right.is_some() {
                    let ri = i + 1;
                    let rj = j + 2usize.pow(height as u32 - i as u32 - 1);

                    dfs(&node.right, res, ri, rj, height);
                }
            }
        }

        dfs(&root, &mut res, 0, (n as usize - 1) / 2, height);

        res
    }
}
