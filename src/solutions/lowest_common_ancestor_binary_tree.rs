// // Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
//
// use std::cell::{Ref, RefCell};
// use std::rc::Rc;
// pub fn lowest_common_ancestor(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     p: Option<Rc<RefCell<TreeNode>>>,
//     q: Option<Rc<RefCell<TreeNode>>>,
// ) -> Option<Rc<RefCell<TreeNode>>> {
//     if p.is_none() || q.is_none() {
//         return None
//     }
//
//     let node_tup = visit(root, p.unwrap(), q.unwrap());
//
//     if node_tup.1 {
//         return node_tup.0
//     }
//
//     None
// }
//
// // visit traverse the tree from the current node and returns true if it
// // finds p or q in any of its left and right subtrees
// fn visit(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     p: Rc<RefCell<TreeNode>>,
//     q: Rc<RefCell<TreeNode>>
// ) -> (Option<Rc<RefCell<TreeNode>>>, bool) {
//     let value = root.borrow().val;
//     println!("{value}");
//
//     let inner = match root {
//         Some(root) => root,
//         None => return (None, false)
//     };
//
//     let inner = inner.borrow();
//
//     if inner.val == p.borrow().val || inner.val == q.borrow().val {
//         return (None, true)
//     }
//
//     let first = if inner.left.is_some() { visit(inner.left.clone(), p.clone(), q.clone()) } else {(None, false)};
//     let second = if inner.right.is_some() { visit(inner.right.clone(), p.clone(), q.clone()) } else {(None, false)};
//
//     if first.1 && second.1 {
//         return (first.0, true)
//     }
//
//     (None, false)
// }
