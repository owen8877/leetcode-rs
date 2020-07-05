use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn from_arr_mask_i32(arr: &[i32], mask: &[i32]) -> Self {
        TreeNode::from_arr(arr, &mask.iter().map(|x| *x == 1).collect::<Vec<bool>>())
    }

    pub fn from_arr(arr: &[i32], mask: &[bool]) -> Self {
        let n = arr.len();
        if n == 0 {
            panic!("The array should not be empty!")
        }
        if mask.len() != n {
            panic!("The array should have the same length as the mask!")
        }

        fn core(index: usize, arr: &[i32], mask: &[bool]) -> TreeNode {
            fn helper(child: usize, arr: &[i32], mask: &[bool]) -> Option<Rc<RefCell<TreeNode>>> {
                if child >= arr.len() || mask[child] {
                    return None
                } else {
                    Some(Rc::new(RefCell::new(core(child, arr, mask))))
                }
            }

            TreeNode {
                val: arr[index],
                left: helper(index*2+1, arr, mask),
                right: helper(index*2+2, arr, mask),
            }
        }

        core(0, arr, mask)
    }
}

#[test]
fn test_from_arr() {
    let treenode = TreeNode::from_arr_mask_i32(
        &[1, 2, 3, 4, 5, 6],
        &[0, 0, 0, 1, 0, 0]
    );
    println!("{:#?}", treenode);
}

