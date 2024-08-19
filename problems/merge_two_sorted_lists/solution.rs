// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() && list2.is_none() {
            return None;
        } else if list1.is_none() {
            return list2;
        } else if list2.is_none() {
            return list1;
        }

        let l1 = list1.unwrap();
        let l2 = list2.unwrap();
        if l1.val < l2.val {
            return Some(Box::new(ListNode {
                val: l1.val,
                next: Solution::merge_two_lists(l1.next, Some(l2)),
            }));
        } else {
            return Some(Box::new(ListNode {
                val: l2.val,
                next: Solution::merge_two_lists(Some(l1), l2.next),
            }));
        }
    }
}
