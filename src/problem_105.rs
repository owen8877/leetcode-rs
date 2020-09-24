use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build_tree_by_slice(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match preorder.len() {
            0 => None,
            n => {
                let val = preorder[0];
                let index_in_inorder = inorder.iter().position(|&r| r == val).unwrap();
                let left = build_tree_by_slice(&preorder[1..=index_in_inorder], &inorder[..index_in_inorder]);
                let right = build_tree_by_slice(&preorder[index_in_inorder + 1..], &inorder[index_in_inorder + 1..]);
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
        }
    }

    build_tree_by_slice(preorder.as_slice(), inorder.as_slice())
}