pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;

        Solution::find(root, &mut res);

        res
    }

    pub fn find(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        // find root.left.depth + root.right.depth, compire with res

        if let Some(node) = root {
            let node = node.borrow();

            let left_node = node.left.as_ref().map(Rc::clone);
            let left_depth = Solution::find(left_node, res);

            let right_node = node.right.as_ref().map(Rc::clone);
            let right_depth = Solution::find(right_node, res);

            let depth = left_depth + right_depth;
            if depth > *res {
                *res = depth;
            }

            return 1 + left_depth.max(right_depth);
        } else {
            return 0;
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{binary_tree::string2tree, solution::Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let data = "[1,2,3,4,5]";
        let root = string2tree(data);

        println!("{:?}", Solution::diameter_of_binary_tree(root));
    }
}
