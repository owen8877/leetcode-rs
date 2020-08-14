use std::rc::Rc;
use std::cell::RefCell;
use crate::treenode::*;

// Reference: https://leetcode.com/problems/maximum-width-of-binary-tree/discuss/727053/DSF-and-BFS-With-Detailed-Explanation-or-Diagram
pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::cmp::max;
    if root.is_none() { return 0 }
    let root = root.as_ref().unwrap();
    let mut left_most_node = vec![];

    fn visit_and_find_longest_width(node: &Rc<RefCell<TreeNode>>, depth: usize, id: i32, left_most_node: &mut Vec<i32>) -> i32 {
        if depth == left_most_node.len() {
            left_most_node.push(id);
        }
        let mut width = id - left_most_node[depth] + 1;
        if let Some(left) = node.borrow().left.as_ref() {
            width = max(width, visit_and_find_longest_width(left, depth+1, id*2+1, left_most_node));
        }
        if let Some(right) = node.borrow().right.as_ref() {
            width = max(width, visit_and_find_longest_width(right, depth+1, id*2+2, left_most_node));
        }
        width
    }

    visit_and_find_longest_width(root, 0, 0, &mut left_most_node)
}

#[test]
fn test_width_of_binary_tree() {
    assert_eq!(width_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode::from_arr_mask_i32(
        &[1, 3, 0, 5, 3],
        &[0, 0, 1, 0, 0]))))), 2)
}