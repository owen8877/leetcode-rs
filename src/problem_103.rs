use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if let Some(root) = root {
        let mut result = vec![];
        let mut queue = vec![root];
        let mut reverse_flag = false;
        while !queue.is_empty() {
            let mut children_queue=  vec![];
            let mut list = vec![];
            for node in queue {
                list.push(node.borrow().val);
                if reverse_flag {
                    if let Some(right) = node.borrow().right.clone() {
                        children_queue.push(right);
                    }
                    if let Some(left) = node.borrow().left.clone() {
                        children_queue.push(left);
                    }
                } else {
                    if let Some(left) = node.borrow().left.clone() {
                        children_queue.push(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        children_queue.push(right);
                    }
                }
            }
            reverse_flag = !reverse_flag;
            children_queue.reverse();
            queue = children_queue;
            result.push(list);
        }
        result
    } else {
        vec![]
    }
}