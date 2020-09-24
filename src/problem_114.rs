use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn core(root: &mut Rc<RefCell<TreeNode>>) {
        let mut r = root.borrow_mut();
        match (r.left.take(), r.right.take()) {
            (None, None) => (),
            (Some(mut left), None) => {
                core(&mut left);
                r.right = Some(left);
            },
            (None, Some(mut right)) => {
                core(&mut right);
                r.right = Some(right);
            },
            (Some(mut left), Some(mut right)) => {
                core(&mut left);
                core(&mut right);

                let mut l = left.clone();
                loop {
                    let n = l.borrow().right.clone();
                    if let Some(k) = n {
                        l = k;
                    } else {
                        break
                    }
                }
                l.borrow_mut().right = Some(right);
                r.right = Some(left);
            },
        }
    }

    root.as_mut().map(|root| core(root));
}

#[test]
fn test_flattern() {
    let mut treenode = Some(Rc::new(RefCell::new(TreeNode::from_arr_mask_i32(
        &[1, 2, 3, 4, 5, 6],
        &[0, 0, 0, 1, 0, 0]
    ))));
    flatten(&mut treenode);
    println!("{:?}", treenode);
}