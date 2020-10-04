use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut indices = vec![];

    fn dfs(result: &mut Vec<i32>, indices: &mut Vec<u32>, depth: usize, index: u32, node: Rc<RefCell<TreeNode>>) {
        if indices.len() <= depth {
            result.push(node.borrow().val);
            indices.push(index);
        } else {
            if indices[depth] < index {
                result[depth] = node.borrow().val;
                indices[depth] = index;
            }
        }
        if let Some(left) = node.borrow().left.clone() {
            dfs(result, indices, depth + 1, index * 2, left);
        }
        if let Some(right) = node.borrow().right.clone() {
            dfs(result, indices, depth + 1, index * 2 + 1, right);
        }
    }

    if let Some(root) = root {
        dfs(&mut result, &mut indices, 0, 0, root);
        result
    } else {
        vec![]
    }
}