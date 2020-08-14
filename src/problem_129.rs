use std::rc::Rc;
use std::cell::RefCell;
use crate::treenode::*;

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::cmp::*;

    fn core(node: &Rc<RefCell<TreeNode>>) -> (Vec<i32>, i32) {
        let left = &node.borrow().left;
        let right = &node.borrow().right;
        if left.is_none() && right.is_none() {
            (vec![1], node.borrow().val)
        } else if left.is_some() && right.is_some() {
            let (arr_l, sum_l) = core(left.as_ref().unwrap());
            let (arr_r, sum_r) = core(right.as_ref().unwrap());

            let len = max(arr_l.len(), arr_r.len());
            let mut arr = vec![0; len];
            for i in 0..len {
                if arr_l.len() + i >= len {
                    arr[i] += arr_l[arr_l.len()+i-len];
                }
                if arr_r.len() + i >= len {
                    arr[i] += arr_r[arr_r.len()+i-len];
                }
            }
            arr.push(0);
            let multiplier = arr.iter().fold(0, |acc, x| acc*10+*x);
            (arr, sum_l + sum_r + multiplier*node.borrow().val)
        } else {
            let (arr_c, sum) = if left.is_some() {
                core(left.as_ref().unwrap())
            } else {
                core(right.as_ref().unwrap())
            };
            let mut arr = arr_c.clone();
            arr.push(0);
            let multiplier = arr.iter().fold(0, |acc, x| acc*10+*x);
            (arr, sum + multiplier*node.borrow().val)
        }
    }

    match &root {
        None => 0,
        Some(r) => core(r).1,
    }
}

#[test]
fn test_sum_numbers() {
    assert_eq!(sum_numbers(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })))), 137);
}