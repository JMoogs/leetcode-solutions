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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => return true,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(t1), Some(t2)) => {
                let same_val = t1.as_ref().borrow().val == t2.as_ref().borrow().val;
                let same_left =
                    Solution::is_same_tree(t1.borrow().left.clone(), t2.borrow().left.clone());
                let same_right =
                    Solution::is_same_tree(t1.borrow().right.clone(), t2.borrow().right.clone());
                return same_val && same_left && same_right;
            }
        }
    }
}