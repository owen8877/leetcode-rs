use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            format!("{}({})({})", root.borrow().val, self.serialize(root.borrow().left.clone()), self.serialize(root.borrow().right.clone()))
        } else {
            "".to_string()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn de(data: &[char]) -> Option<Rc<RefCell<TreeNode>>> {
            if data.len() == 0 {
                None
            } else {
                let first_left_p = data.iter().position(|&c| c == '(').unwrap();
                let mut first_right_mp = 0;
                let mut counter = 0;
                for (i, &c) in data.iter().enumerate().skip(first_left_p) {
                    if c == '(' {
                        counter += 1;
                    } else if c == ')' {
                        counter -= 1;
                    } else {
                        continue;
                    }
                    if counter == 0 {
                        first_right_mp = i;
                        break;
                    }
                }

                let val = if data[0] == '-' {
                    -data[1..first_left_p].iter().fold(0, |acc, &x| acc * 10 + (x as u8 - '0' as u8) as i32)
                } else {
                    data[..first_left_p].iter().fold(0, |acc, &x| acc * 10 + (x as u8 - '0' as u8) as i32)
                };
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: de(&data[first_left_p + 1..first_right_mp]),
                    right: de(&data[first_right_mp + 2..data.len() - 1]),
                })))
            }
        }

        let cc: Vec<char> = data.chars().collect();
        de(&cc)
    }
}