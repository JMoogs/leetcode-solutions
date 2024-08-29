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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = Vec::new();
        traverse(&root, &mut v);
        v.sort();
        v.dedup();
        match v.get(1) {
            None => return -1,
            Some(x) => *x,
        }
    }
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    if let Some(val) = root {
        traverse(&val.borrow().left, v);
        v.push(val.borrow().val);
        traverse(&val.borrow().right, v);
    }
}
