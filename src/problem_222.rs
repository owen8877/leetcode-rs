use crate::treenode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn core(root: &Rc<RefCell<TreeNode>>, counter: i32, h: i32) -> i32 {
        if h == 0 {
            return counter
        }

        if let Some(right) = &root.borrow().right {
            if height(right, 0) == h - 1 {
                return core(right, counter * 2 + 1, h - 1)
            }
        }

        core(&root.borrow().left.as_ref().unwrap(), counter * 2, h - 1)
    }

    fn height(root: &Rc<RefCell<TreeNode>>, h: i32) -> i32 {
        match &root.borrow().left {
            None => h,
            Some(left) => height(left, h+1),
        }
    }

    match root.as_ref() {
        None => 0,
        Some(root) => {
            let h = height(root, 0);
            core(root, 1, h)
        },
    }
}

#[test]
fn test_count_nodes() {
    assert_eq!(count_nodes(None), 0);
    assert_eq!(count_nodes(Some(Rc::new(RefCell::new(TreeNode::new(1))))), 1);
    assert_eq!(count_nodes(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })))), 4);
}