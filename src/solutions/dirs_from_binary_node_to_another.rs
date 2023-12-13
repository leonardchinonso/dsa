/// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/solutions/1612105/3-steps/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn _new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn _get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    let (mut start_path, mut dest_path) = (String::new(), String::new());
    match root {
        None => {
            return "".to_string();
        }
        Some(root) => {
            _traverse_from_root(Some(Rc::clone(&root)), start_value, &mut start_path);
            _traverse_from_root(Some(Rc::clone(&root)), dest_value, &mut dest_path);

            // let mut idx = 0_usize;
            // let bound = max(start_path.len(), dest_path.len());
            let start_vector = start_path.chars().collect::<Vec<char>>();
            let dest_vector = dest_path.chars().collect::<Vec<char>>();
            // while idx < bound && start_vector[start_path.len() - idx - 1] == dest_vector[start_path.len() - idx - 1] {
            //     idx += 1;
            // }

            while !start_path.is_empty()
                && !dest_path.is_empty()
                && start_vector.last().unwrap() != dest_vector.last().unwrap()
            {
                start_path.pop();
                dest_path.pop();
            }

            format!(
                "{}{}",
                "U".repeat(start_path.len()),
                dest_path.chars().rev().collect::<String>()
            )

            // let d_rev = dest_path.chars().rev().collect::<String>();
            // format!("{}{}", "U".repeat(start_path.len() - idx), &d_rev[..d_rev.char_indices().nth(idx).unwrap().0])
        }
    }
}

fn _traverse_from_root(node: Option<Rc<RefCell<TreeNode>>>, value: i32, path: &mut String) -> bool {
    if let Some(node) = node {
        let node_ref = node.borrow();
        if node_ref.val == value {
            return true;
        }
        if _traverse_from_root(node_ref.left.clone(), value, path) {
            path.push('L');
        } else if _traverse_from_root(node_ref.right.clone(), value, path) {
            path.push('R');
        }
        return !path.is_empty();
    }
    false
}
