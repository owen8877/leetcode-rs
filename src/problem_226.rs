use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.map(|root| {
        {
            let mut r = root.borrow_mut();
            let left = r.left.take();
            let right = r.right.take();
            r.left = Self::invert_tree(right);
            r.right = Self::invert_tree(left);
        }
        root
    })
}