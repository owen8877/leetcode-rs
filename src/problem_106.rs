use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build_tree_by_slice(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match postorder.len() {
            0 => None,
            n => {
                let val = postorder[n - 1];
                let index_in_inorder = inorder.iter().position(|&r| r == val).unwrap();
                let left = build_tree_by_slice(&inorder[..index_in_inorder], &postorder[..index_in_inorder]);
                let right = build_tree_by_slice(&inorder[index_in_inorder + 1..], &postorder[index_in_inorder..n - 1]);
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
        }
    }

    build_tree_by_slice(inorder.as_slice(), postorder.as_slice())
}
