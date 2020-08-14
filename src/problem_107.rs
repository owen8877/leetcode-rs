use std::rc::Rc;
use std::cell::{RefCell, Ref};
use crate::treenode::*;

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![]
    }

    fn height(root: &Rc<RefCell<TreeNode>>, h: usize) -> usize {
        match &root.borrow().left {
            None => match &root.borrow().right {
                None => h,
                Some(right) => height(right, h+1),
            },
            Some(left) => match &root.borrow().right {
                None => height(left, h+1),
                Some(right) => std::cmp::max(height(left, h+1), height(right, h+1)),
            },
        }
    }
    let h = height(root.as_ref().unwrap(), 1);

    let mut result = vec![vec![]; h];

    fn core(node: &Rc<RefCell<TreeNode>>, result: &mut Vec<Vec<i32>>, depth: usize) {
        result[depth].push(node.borrow().val);
        if let Some(left) = &node.borrow().left {
            core(left, result, depth+1);
        }
        if let Some(right) = &node.borrow().right {
            core(right, result, depth+1);
        }
    }

    core(root.as_ref().unwrap(), &mut result, 0);
    result.reverse();
    result
}

#[test]
fn test_level_order_bottom() {
    assert_eq!(level_order_bottom(None), vec![vec![]]);
    assert_eq!(level_order_bottom(Some(Rc::new(RefCell::new(TreeNode::from_arr_mask_i32(
        &[3, 9, 20, 0, 0, 15, 7],
        &[0, 0, 0, 1, 1, 0, 0]
    ))))), vec![vec![15,7], vec![9,20], vec![3]]);
}