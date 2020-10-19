use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn core(root: Option<Rc<RefCell<TreeNode>>>, k: usize) -> (Option<i32>, usize) {
        root.map_or((None, 0), |r| {
            let (lfind, lnum) = core(r.borrow().left.clone(), k);
            if let Some(num) = lfind {
                return (Some(num), 0);
            }
            if lnum + 1 == k {
                return (Some(r.borrow().val), 0);
            }
            let (rfind, rnum) = core(r.borrow().right.clone(), k - lnum - 1);
            if let Some(num) = rfind {
                (Some(num), 0)
            } else {
                (None, lnum + 1 + rnum)
            }
        })
    }

    core(root, k as usize).0.unwrap()
}