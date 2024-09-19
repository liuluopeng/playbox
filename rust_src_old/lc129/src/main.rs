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

struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut paths = vec![];
        let mut path = vec![];

        Self::find(root, &mut paths, path);


        for path in paths {
            let mut one_num: String  = String::from("");
            for i in path {
                one_num.push_str(  &i.to_string() )
            }
            let n:i32  =  one_num.parse().expect("msg");
            res += n;
        }


        res
    }

    pub fn find(node: Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<Vec<i32>>, path: Vec<i32>) {
        match node {
            Some(node) => {
                let node = node.borrow();

                let mut path = path.clone();
                path.push(node.val);

                let path2 = path.clone();

                if node.left.is_none() && node.right.is_none() {
                    // 左右都空
                    // 把一条路径添加
                    paths.push(path.to_vec());
                    return;
                }

                Self::find(node.left.clone(), paths, path);
                Self::find(node.right.clone(), paths, path2);
            }

            None => {}
        }
    }
}

fn main() {
    // 创建节点
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));

    // 构建树结构
    node1.borrow_mut().left = Some(node2.clone());
    node1.borrow_mut().right = Some(node3.clone());

    // 输出树结构
    println!("树结构: {:?}", node1);

    println!("{:?}", Solution::sum_numbers(Some(node1)));
}
