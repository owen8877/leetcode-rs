pub fn calculate(s: String) -> i32 {
    let mut cn = 0;
    let mut stack = vec![];
    let mut op = '+';
    for c in s.chars() {
        match c {
            '0'..='9' => {
                cn *= 10;
                cn += (c as u8 - '0' as u8) as i32;
            }
            ' ' => {}
            _ => {
                match op {
                    '+' => {
                        stack.push(cn);
                    }
                    '-' => {
                        stack.push(-cn);
                    }
                    '*' => {
                        let pn = stack.pop().unwrap();
                        stack.push(pn * cn);
                    }
                    '/' => {
                        let pn = stack.pop().unwrap();
                        stack.push(pn / cn);
                    }
                    _ => {}
                }
                op = c;
                cn = 0;
            }
        }
    }
    match op {
        '+' => {
            stack.push(cn);
        }
        '-' => {
            stack.push(-cn);
        }
        '*' => {
            let pn = stack.pop().unwrap();
            stack.push(pn * cn);
        }
        '/' => {
            let pn = stack.pop().unwrap();
            stack.push(pn / cn);
        }
        _ => {}
    }
    stack.iter().fold(0, |acc, &x| acc + x)
}