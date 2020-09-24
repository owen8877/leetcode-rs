use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::cmp::min;
    root.map_or(0, |node| {
        fn core(node: Rc<RefCell<TreeNode>>) -> i32 {
            let mut depth = None;
            if let Some(left) = node.borrow().left.clone() {
                depth = Some(min(depth.unwrap_or(1 << 30), core(left) + 1));
            }
            if let Some(right) = node.borrow().right.clone() {
                depth = Some(min(depth.unwrap_or(1 << 30), core(right) + 1));
            }
            depth.unwrap_or(1)
        }

        core(node)
    })
}