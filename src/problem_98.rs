pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_valid_with_bounds(root: Rc<RefCell<TreeNode>>, min: i64, max: i64) -> bool {
        let val = root.borrow().val as i64;
        if val >= max || val <= min {
            return false
        }
        if let Some(left) = root.borrow().left.clone() {
            if !is_valid_with_bounds(left, min, val) {
                return false
            }
        }
        if let Some(right) = root.borrow().right.clone() {
            if !is_valid_with_bounds(right, val, max) {
                return false
            }
        }
        true
    }

    if let Some(root) = root {
        is_valid_with_bounds(root, (-1) << 32, 1 << 32)
    } else {
        true
    }
}