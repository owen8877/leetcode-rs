use std::cell::RefCell;
use std::rc::Rc;

use crate::treenode::TreeNode;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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