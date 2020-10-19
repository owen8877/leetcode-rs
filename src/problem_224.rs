use Unit::*;

#[derive(Debug)]
enum Unit {
    Op(char),
    Num(i32),
}

pub fn calculate(s: String) -> i32 {
    fn eval(stack: &mut Vec<Unit>) {
        let mut tmp = None;
        let mut tmpc = '+';
        while let Some(unit) = stack.pop() {
            match unit {
                Num(i) => {
                    if let Some(aa) = tmp {
                        tmp = Some(match tmpc {
                            '+' => aa + i,
                            '-' => aa - i,
                            _ => panic!(),
                        });
                    } else {
                        tmp = Some(i);
                    }
                }
                Op(c) => {
                    if c == ')' {
                        break;
                    }
                    tmpc = c;
                }
            }
        }
        stack.push(Num(tmp.unwrap()))
    }

    let mut tmp_digit = 0;
    let mut tmp_order = 0;
    let mut stack = vec![Op(')')];
    for c in s.chars().rev() {
        if '0' <= c && c <= '9' {
            tmp_digit += (c as u8 - '0' as u8) as i32 * 10i32.pow(tmp_order);
            tmp_order += 1;
        } else if c != ' ' {
            stack.push(Num(tmp_digit));
            tmp_digit = 0;
            tmp_order = 0;
            stack.push(Op(c));
            if c == '(' {
                stack.pop();
                eval(&mut stack);
            }
        }
    }
    stack.push(Num(tmp_digit));
    eval(&mut stack);
    if let Num(i) = stack[0] { i } else { 0 }
}