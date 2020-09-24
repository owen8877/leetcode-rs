use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nums = vec![];
    let mut p = head.as_ref();
    while let Some(q) = p {
        nums.push(q.val);
        p = q.next.as_ref();
    }

    fn core(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            n => {
                let mid = n / 2;
                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[mid],
                    left: core(&nums[..mid]),
                    right: core(&nums[mid + 1..]),
                })))
            }
        }
    }

    core(nums.as_slice())
}