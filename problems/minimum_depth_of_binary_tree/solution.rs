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
 pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(t) = root {
            if t.borrow().left.is_none() && t.borrow().right.is_none() {
                return 1;
            }
            if t.borrow().left.is_none() {
                return 1 + Solution::min_depth(t.borrow().right.clone());
            }
            if t.borrow().right.is_none() {
                return 1 + Solution::min_depth(t.borrow().left.clone());
            }

            let ml = Solution::min_depth(t.borrow().left.clone());
            let mr = Solution::min_depth(t.borrow().right.clone());

            if ml > mr {
                return 1 + mr;
            } else {
                return 1 + ml;
            }

        } else {
            return 0;
        }
    }
}
