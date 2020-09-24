use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::cmp::max;
    root.map_or(0, |node| max(Self::max_depth(node.borrow().left.clone()), Self::max_depth(node.borrow().right.clone())) + 1)
}