use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

// Credits: https://leetcode.com/problems/delete-node-in-a-bst/discuss/821707/Java-Recursive-Solution-with-Detailed-Algorithm-Steps
pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    use std::cmp::Ordering::*;
    fn successor(root: Rc<RefCell<TreeNode>>) -> i32 {
        if let Some(left) = root.borrow().left.clone() {
            successor(left)
        } else {
            root.borrow().val
        }
    }

    root.map(|r| {
        {
            let mut rm = r.borrow_mut();
            let cmp = rm.val.cmp(&key);
            match cmp {
                Equal => {
                    if rm.left.is_none() {
                        return rm.right.clone();
                    } else if rm.right.is_none() {
                        return rm.left.clone();
                    }
                    rm.val = successor(rm.right.clone().unwrap());
                    rm.right = Self::delete_node(rm.right.clone(), rm.val);
                }
                Greater => {
                    rm.left = Self::delete_node(rm.left.clone(), key);
                }
                Less => {
                    rm.right = Self::delete_node(rm.right.clone(), key);
                }
            };
        }
        Some(r)
    }).flatten()
}