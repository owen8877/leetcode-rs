use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = root.unwrap();
    let p = p.unwrap();
    let q = q.unwrap();

    fn find(root: Option<Rc<RefCell<TreeNode>>>, p: Rc<RefCell<TreeNode>>, history: &mut Vec<bool>) -> bool {
        root.map_or(false, |root| {
            if root.borrow().val == p.borrow().val {
                return true;
            }
            history.push(true);
            if find(root.borrow().left.clone(), p.clone(), history) {
                return true;
            }
            history.pop();
            history.push(false);
            if find(root.borrow().right.clone(), p.clone(), history) {
                return true;
            }
            history.pop();
            false
        })
    }

    let mut history_p = vec![];
    find(Some(root.clone()), p, &mut history_p);
    let mut history_q = vec![];
    find(Some(root.clone()), q, &mut history_q);

    if history_p.len() == 0 || history_q.len() == 0 {
        return Some(root.clone());
    }
    let mut p = root.clone();
    for i in 0..std::cmp::min(history_p.len(), history_q.len()) {
        if history_p[i] != history_q[i] {
            break;
        }
        p = match history_p[i] {
            true => p.borrow().left.clone().unwrap(),
            false => p.borrow().right.clone().unwrap(),
        };
    }
    Some(p)
}