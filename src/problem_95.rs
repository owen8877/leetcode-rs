use std::rc::Rc;
use std::cell::RefCell;
use crate::treenode::TreeNode;

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn generate_lr_trees(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l > r {
            vec![None]
        } else {
            (l..=r).flat_map(|i| {
                let ltrees = generate_lr_trees(l, i-1);
                let rtrees = generate_lr_trees(i+1, r);
                ltrees.iter().flat_map(|ltree| {
                    rtrees.iter().map(|rtree| {
                        Some(Rc::new(RefCell::new(TreeNode {val: i, left: ltree.clone(), right: rtree.clone()})))
                    }).collect::<Vec<Option<Rc<RefCell<TreeNode>>>>>()
                }).collect::<Vec<Option<Rc<RefCell<TreeNode>>>>>()
            }).collect()
        }
    }

    if n < 1 {
        vec![]
    } else {
        generate_lr_trees(1, n)
    }
}

#[test]
fn test_generate_trees() {
    generate_trees(2);
}