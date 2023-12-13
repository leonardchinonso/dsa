/// https://leetcode.com/problems/binary-tree-pruning/
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }

    let inner = root.unwrap();
    let node_ref = inner.borrow();

    // if it is a leaf node, return a null node
    if node_ref.val == 0 && node_ref.left.is_none() && node_ref.right.is_none() {
        return None;
    }

    // free the memory occupied by the node reference
    drop(node_ref);

    // visit the left and right nodes
    let mut node_mut = inner.borrow_mut();
    node_mut.left = prune_tree(node_mut.left.clone());
    node_mut.right = prune_tree(node_mut.right.clone());

    if node_mut.val == 0 && node_mut.left.is_none() && node_mut.right.is_none() {
        None
    } else {
        Some(inner.clone())
    }
}
