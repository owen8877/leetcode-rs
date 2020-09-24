use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    fn core(root: Rc<RefCell<TreeNode>>, sum: i32) -> bool {
        let remainder = sum - root.borrow().val;
        let mut is_leaf = false;
        if let Some(left) = root.borrow().left.clone() {
            if core(left, remainder) {
                return true;
            }
            is_leaf = true;
        }
        if let Some(right) = root.borrow().right.clone() {
            if core(right, remainder) {
                return true;
            }
            is_leaf = true;
        }
        !is_leaf && remainder == 0
    }

    root.map_or(false, |root| core(root, sum))
}