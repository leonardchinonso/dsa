/// https://leetcode.com/problems/balance-a-binary-search-tree/
///
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

mod to_array_approach {
    use super::*;

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut bst_vector = Vec::new();
        to_inorder_array(root, &mut bst_vector);
        let root = to_bst(&bst_vector, 0, bst_vector.len() as i32 - 1);
        drop(bst_vector);
        root
    }

    fn to_inorder_array(
        node: Option<Rc<RefCell<TreeNode>>>,
        nodes: &mut Vec<Rc<RefCell<TreeNode>>>,
    ) {
        if node.is_none() {
            return;
        }

        let actual_node = node.unwrap();
        let actual_node_ref = actual_node.borrow();

        to_inorder_array(actual_node_ref.left.clone(), nodes);
        nodes.push(Rc::clone(&actual_node));
        to_inorder_array(actual_node_ref.right.clone(), nodes);
    }

    fn to_bst(
        nodes: &[Rc<RefCell<TreeNode>>],
        start: i32,
        end: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }

        let root_idx = ((start + end) / 2);
        let root = nodes[root_idx as usize].clone();

        // TODO check for underflow
        root.borrow_mut().left = to_bst(nodes, start, root_idx - 1);
        root.borrow_mut().right = to_bst(nodes, root_idx + 1, end);

        Some(root)
    }
}
