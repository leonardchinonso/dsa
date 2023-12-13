// // Definition for singly-linked list.
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
//
// pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     None
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     fn create_linked_list() -> Option<Box<ListNode>> {
//         let values = vec![4, 2, 8, 10, 2, 1, 0, 19];
//         let mut head: Option<Box<ListNode>> = None;
//         let mut prev: Option<Box<ListNode>> = None;
//         for value in values {
//             let node = Box::new(ListNode::new(value));
//             match prev {
//                 Some(mut prev_inner) => {
//                     prev_inner.next = Some(node.clone());
//                     prev = Some(node.clone());
//                 },
//                 None => { head = Some(node); }
//             }
//         }
//         head
//     }
//
//     #[test]
//     fn sort_list_works() {
//         let list = create_linked_list();
//         sort_list(list.clone());
//         let mut curr = list.clone();
//         let mut prev: Option<Box<ListNode>> = None;
//         loop {
//             prev = curr.clone();
//             let curr = curr.unwrap().next;
//             if curr.is_none() { // move occurs after write to unwrap
//                 break
//             }
//             if prev.is_some() && curr.is_some() {
//                 assert!(prev.unwrap().val <= curr.unwrap().val)
//             }
//         }
//     }
// }
