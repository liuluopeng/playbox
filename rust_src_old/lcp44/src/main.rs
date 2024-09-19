fn main() {
    println!("Hello, world!");
}


// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn num_color(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	let mut exist = vec![0;1008];
        Self::dfs(root, &mut exist);

	let mut counter = 0;
	for i in exist {
		if i>0 {
			counter += 1;
		}
	}
	counter 	
    }
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, exist: &mut Vec<usize>)  {
  	if let Some(node) = root {
		let node = node.borrow();
		exist[node.val as usize] += 1;
		Self::dfs(node.left.clone(), exist);
		Self::dfs(node.right.clone(), exist);
 
   }
}
