use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn height_and_balanced(node: Option<Rc<RefCell<TreeNode>>>) -> (usize, bool) {
        node.map_or((0, true), |node| {
            let (lh, lb) = height_and_balanced(node.borrow().left.clone());
            if !lb {
                return (0, false);
            }
            let (rh, rb) = height_and_balanced(node.borrow().right.clone());
            if !rb {
                return (0, false);
            }
            if lh == rh {
                (lh + 1, true)
            } else if lh == rh + 1 {
                (lh + 1, true)
            } else if rh == lh + 1 {
                (rh + 1, true)
            } else {
                (0, false)
            }
        })
    }

    height_and_balanced(root).1
}