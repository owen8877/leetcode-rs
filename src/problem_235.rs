use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = root.unwrap();
    let p = p.unwrap();
    let q = q.unwrap();

    fn find(root: Option<Rc<RefCell<TreeNode>>>, p: Rc<RefCell<TreeNode>>, index: usize) -> Option<usize> {
        root.map_or(None, |root| {
            if root.borrow().val == p.borrow().val {
                return Some(index);
            }
            if let Some(a) = find(root.borrow().left.clone(), p.clone(), index * 2) {
                return Some(a);
            }
            if let Some(a) = find(root.borrow().right.clone(), p.clone(), index * 2 + 1) {
                return Some(a);
            }
            None
        })
    }

    let path_p = find(Some(root.clone()), p, 1).unwrap();
    let path_q = find(Some(root.clone()), q, 1).unwrap();
    if path_p == 0 || path_q == 0 {
        return Some(root.clone());
    }
    let (mut pp, mut pq) = if path_p > path_q {
        (path_p, path_q)
    } else {
        (path_q, path_p)
    };
    let mut tp = ((pp as f64).log2().floor() as usize);
    let mut tq = ((pq as f64).log2().floor() as usize);
    while tp > tq {
        pp /= 2;
        tp -= 1;
    }
    while pp != pq {
        pp /= 2;
        pq /= 2;
        tp -= 1;
    }
    let mut p = root.clone();
    for i in 0..tp {
        p = match (pp >> (tp - i - 1)) & 1 {
            0 => p.borrow().left.clone().unwrap(),
            _ => p.borrow().right.clone().unwrap(),
        };
    }
    Some(p)
}