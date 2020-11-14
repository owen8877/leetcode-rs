use std::cell::RefCell;
use std::cmp::*;
use std::rc::Rc;

use crate::treenode::TreeNode;

type ONode = Option<Rc<RefCell<TreeNode>>>;
type RNode = Rc<RefCell<TreeNode>>;

pub fn rob(root: ONode) -> i32 {
    fn search(root: RNode) -> (i32, i32) {
        let val = root.borrow().val;
        let left_max = root.borrow().left.clone().map_or((0, 0), |left| search(left));
        let right_max = root.borrow().right.clone().map_or((0, 0), |right| search(right));

        let nomask = val + left_max.0 + right_max.0;
        let mask = left_max.1 + right_max.1;
        (mask, max(nomask, mask))
    }

    root.map_or(0, |r| {
        let (a, b) = search(r);
        max(a, b)
    })
}