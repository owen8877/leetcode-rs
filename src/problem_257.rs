use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn vec2str(v: &Vec<i32>) -> String {
        let mut s = format!("{}", v[0]);
        for i in 1..v.len() {
            s += format!("->{}", v[i]).as_str();
        }
        s
    }

    let mut history = vec![];
    let mut result = vec![];

    fn visit(root: Rc<RefCell<TreeNode>>, history: &mut Vec<i32>, result: &mut Vec<String>) {
        history.push(root.borrow().val);

        let mut is_leaf = true;
        if let Some(left) = root.borrow().left.clone() {
            is_leaf = false;
            visit(left, history, result);
        }
        if let Some(right) = root.borrow().right.clone() {
            is_leaf = false;
            visit(right, history, result);
        }
        if is_leaf {
            result.push(vec2str(history));
        }

        history.pop();
    }

    if let Some(root) = root {
        visit(root, &mut history, &mut result);
    }
    result
}