use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
    fn core(root: Rc<RefCell<TreeNode>>, sum: i32, prefix: &mut Vec<i32>, storage: &mut Vec<Vec<i32>>) {
        let remainder = sum - root.borrow().val;
        let mut is_leaf = false;
        prefix.push(root.borrow().val);
        if let Some(left) = root.borrow().left.clone() {
            core(left, remainder, prefix, storage);
            is_leaf = true;
        }
        if let Some(right) = root.borrow().right.clone() {
            core(right, remainder, prefix, storage);
            is_leaf = true;
        }
        if !is_leaf && remainder == 0 {
            storage.push(prefix.clone());
        }
        prefix.pop();
    }

    let mut prefix = vec![];
    let mut storage = vec![];
    root.map(|root| core(root, sum, &mut prefix, &mut storage));
    storage
}