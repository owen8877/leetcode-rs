use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_mirror(l: Rc<RefCell<TreeNode>>, r: Rc<RefCell<TreeNode>>) -> bool {
        if l.borrow().val != r.borrow().val {
            return false
        }
        if let Some(ll) = l.borrow().left.clone() {
            if let Some(rr) = r.borrow().right.clone() {
                if !is_mirror(ll, rr) {
                    return false
                }
            } else {
                return false
            }
        } else {
            if r.borrow().right.is_some() {
                return false
            }
        }

        if let Some(lr) = l.borrow().right.clone() {
            if let Some(rl) = r.borrow().left.clone() {
                is_mirror(lr, rl)
            } else {
                false
            }
        } else {
            r.borrow().left.is_none()
        }
    }

    if let Some(root_ref) = root {
        if let Some(l) = root_ref.borrow().left.clone() {
            if let Some(r) = root_ref.borrow().right.clone() {
                is_mirror(l, r)
            } else {
                false
            }
        } else {
            root_ref.borrow().right.is_none()
        }
    } else {
        true
    }
}