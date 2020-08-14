use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

// Follow-up needs attention.
// Reference: https://leetcode.com/problems/recover-binary-search-tree/discuss/781762/C%2B%2B-Straightforward-DFS-solution
pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn update(incorrect_pair: (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>), small: Rc<RefCell<TreeNode>>, large: Rc<RefCell<TreeNode>>) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        let (s, l) = incorrect_pair;
        (
            Some(s.map_or(small.clone(), |s| {
                if s.borrow().val < small.borrow().val {
                    s
                } else {
                    small
                }
            })),
            Some(l.map_or(large.clone(), |l| {
                if l.borrow().val > large.borrow().val {
                    l
                } else {
                    large
                }
            })),
        )
    }

    fn visit_and_replace(root: Rc<RefCell<TreeNode>>, mut incorrect_pair: (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>), left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(left) = left.clone() {
            if root.borrow().val < left.borrow().val {
                incorrect_pair = update(incorrect_pair, root.clone(), left.clone());
            }
        }
        if let Some(l) = root.borrow().left.clone() {
            incorrect_pair = visit_and_replace(l, incorrect_pair, left.clone(), Some(root.clone()));
        }
        if let Some(right) = right.clone() {
            if right.borrow().val < root.borrow().val {
                incorrect_pair = update(incorrect_pair, right.clone(), root.clone());
            }
        }
        if let Some(r) = root.borrow().right.clone() {
            incorrect_pair = visit_and_replace(r, incorrect_pair, Some(root.clone()), right.clone());
        }

        incorrect_pair
    }

    let pair = visit_and_replace(root.clone().unwrap(), (None, None), None, None);

    let node1 = pair.0.unwrap();
    let node2 = pair.1.unwrap();
    let tmp = node1.borrow().val;
    node1.borrow_mut().val = node2.borrow().val;
    node2.borrow_mut().val = tmp;
}