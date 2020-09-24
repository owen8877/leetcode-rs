use std::cell::RefCell;
use std::cmp::*;
use std::collections::*;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn max_path_sum2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_connect_path_sum(node: Rc<RefCell<TreeNode>>, index: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        if cache.contains_key(&index) {
            return *cache.get(&index).unwrap();
        }
        let left_max = if let Some(left) = node.borrow().left.clone() {
            max_connect_path_sum(left, index * 2 + 1, cache)
        } else { 0 };
        let right_max = if let Some(right) = node.borrow().right.clone() {
            max_connect_path_sum(right, index * 2 + 2, cache)
        } else { 0 };
        let v = node.borrow().val;
        let m = max(max(max(0, v), v + left_max), v + right_max);
        cache.insert(index, m);
        m
    }

    fn max_sum(node: Rc<RefCell<TreeNode>>, index: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        let (left_max, l_sum) = if let Some(left) = node.borrow().left.clone() {
            (Some(max_sum(left.clone(), index * 2 + 1, cache)), max_connect_path_sum(left, index * 2 + 1, cache))
        } else { (None, 0) };
        let (right_max, r_sum) = if let Some(right) = node.borrow().right.clone() {
            (Some(max_sum(right.clone(), index * 2 + 2, cache)), max_connect_path_sum(right, index * 2 + 2, cache))
        } else { (None, 0) };
        let v = node.borrow().val;
        let mut result = v + l_sum + r_sum;
        if let Some(l) = left_max {
            result = max(l, result);
        }
        if let Some(r) = right_max {
            result = max(r, result);
        }
        result
    }

    let mut cache = HashMap::new();
    root.map_or(0, |r| max_sum(r, 0, &mut cache))
}

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_connect_path_sum(node: Rc<RefCell<TreeNode>>) -> i32 {
        let left_max = node.borrow().left.clone().map_or(0, |left| max_connect_path_sum(left));
        let right_max = node.borrow().right.clone().map_or(0, |right| max_connect_path_sum(right));
        let v = node.borrow().val;
        max(max(max(0, v), v + left_max), v + right_max)
    }

    fn max_sum(node: Rc<RefCell<TreeNode>>) -> i32 {
        let (left_max, l_sum) = if let Some(left) = node.borrow().left.clone() {
            (Some(max_sum(left.clone())), max_connect_path_sum(left))
        } else { (None, 0) };
        let (right_max, r_sum) = if let Some(right) = node.borrow().right.clone() {
            (Some(max_sum(right.clone())), max_connect_path_sum(right))
        } else { (None, 0) };
        let v = node.borrow().val;
        let mut result = v + l_sum + r_sum;
        if let Some(l) = left_max {
            result = max(l, result);
        }
        if let Some(r) = right_max {
            result = max(r, result);
        }
        result
    }

    root.map_or(0, |r| max_sum(r))
}