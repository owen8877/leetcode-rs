pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];
    if let Some(root) = root {
        let mut current = Some(root.clone());
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }
            let node = stack.pop().unwrap();
            result.push(node.borrow().val);
            current = node.borrow().right.clone();
        }
    }
    result
}
