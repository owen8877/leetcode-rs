use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if let Some(root) = root {
        let mut result = vec![];
        let mut nodes = vec![root];

        while !nodes.is_empty() {
            result.push(nodes.clone());
            let mut children = vec![];
            for node in nodes {
                if let Some(left) = node.borrow().left.clone() {
                    children.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    children.push(right);
                }
            }
            nodes = children;
        }

        result.into_iter().map(|v| v.into_iter().map(|node| node.borrow().val).collect::<Vec<i32>>()).collect()
    } else {
        vec![]
    }
}