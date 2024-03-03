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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut start_node = match head {
            Some(n) => n,
            None => return None,
        };

        let mut current = &mut start_node;

        loop {
            match &current.next {
                Some(next_node) => {
                    if current.val == next_node.val {
                        current.next = next_node.next.clone();
                        continue;
                    }
                }
                None => return Some(start_node),
            }

            current = current.next.as_mut().unwrap();
        }
    }
}