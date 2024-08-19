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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            // If not found return null
            None => return None,
            Some(tree) => {
                let checkval = tree.borrow().val;

                if checkval == val {
                    return Some(tree)
                }

                if checkval > val {
                    Solution::search_bst(tree.borrow().left.clone(), val)
                } else {
                    Solution::search_bst(tree.borrow().right.clone(), val)
                }
            },
        }
    }
}