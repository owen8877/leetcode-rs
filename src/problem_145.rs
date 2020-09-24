use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn core(root: Rc<RefCell<TreeNode>>, v: &mut Vec<i32>) {
        if let Some(left) = root.borrow().left.clone() {
            core(left, v);
        }
        if let Some(right) = root.borrow().right.clone() {
            core(right, v);
        }
        v.push(root.borrow().val);
    }

    let mut v = vec![];
    root.map(|r| core(r, &mut v));
    v
}