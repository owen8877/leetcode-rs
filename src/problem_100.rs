pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn core(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(p_rc) = p {
            if let Some(q_rc) = q {
                p_rc.borrow().val == q_rc.borrow().val
                    && core(p_rc.borrow().left.as_ref(), q_rc.borrow().left.as_ref())
                    && core(p_rc.borrow().right.as_ref(), q_rc.borrow().right.as_ref())
            } else {
                false
            }
        } else {
            q.is_none()
        }
    }
    core(p.as_ref(), q.as_ref())
}