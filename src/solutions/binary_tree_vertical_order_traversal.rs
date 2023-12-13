/// https://leetcode.com/problems/binary-tree-vertical-order-traversal/description/
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
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

struct NodeTuple(Rc<RefCell<TreeNode>>, i32);

impl NodeTuple {
    fn new(node: Rc<RefCell<TreeNode>>, col_pos: i32) -> Self {
        Self(node, col_pos)
    }
}

pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut queue: VecDeque<NodeTuple> = VecDeque::new();
    let mut next_queue: VecDeque<NodeTuple> = VecDeque::new();
    let mut hashmap = HashMap::new();
    let mut results: Vec<Vec<i32>> = Vec::new();
    let (mut lowest_col_idx, mut highest_col_idx) = (0i32, 0i32);

    let root = root.unwrap();
    // let root_inner = root.into_inner();
    queue.push_back(NodeTuple::new(root, 0i32));

    while !queue.is_empty() {
        let popped = queue.pop_front().unwrap();
        let (node, col_pos) = (popped.0, popped.1);
        let node_ref = node.borrow();

        lowest_col_idx = min(lowest_col_idx, col_pos);
        highest_col_idx = max(highest_col_idx, col_pos);

        hashmap
            .entry(col_pos)
            .and_modify(|v: &mut Vec<i32>| v.push(node_ref.val))
            .or_insert(vec![node_ref.val]);

        if node_ref.left.is_some() {
            let left = node_ref.left.clone().unwrap();
            next_queue.push_back(NodeTuple::new(left, col_pos - 1));
        }
        if node_ref.right.is_some() {
            let right = node_ref.right.clone().unwrap();
            next_queue.push_back(NodeTuple::new(right, col_pos + 1));
        }

        drop(node_ref);

        if queue.len() == 0 {
            queue = next_queue;
            next_queue = VecDeque::new();
        }
    }

    for i in lowest_col_idx..=highest_col_idx {
        results.push(hashmap.remove(&i).take().unwrap())
    }

    results
}

#[cfg(test)]
mod test {
    #[test]
    fn vertical_order_compiles() {}
}
