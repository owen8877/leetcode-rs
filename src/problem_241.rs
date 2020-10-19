use std::collections::*;

use Unit::*;

#[derive(Debug)]
enum Unit {
    Op(char),
    Num(i32),
}

pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
    let mut tmp = 0;
    let mut ops = vec![];
    for c in input.chars() {
        match c {
            '0'..='9' => {
                tmp *= 10;
                tmp += (c as u8 - '0' as u8) as i32;
            }
            _ => {
                ops.push(Num(tmp));
                ops.push(Op(c));
                tmp = 0;
            }
        }
    }
    ops.push(Num(tmp));

    fn core(ops: &[Unit]) -> Vec::<i32> {
        let mut result = Vec::new();
        if ops.len() == 1 {
            if let Num(b) = ops[0] {
                result.push(b);
            }
        } else {
            for i in 0..ops.len() / 2 {
                let lr = core(&ops[..2 * i + 1]);
                let rr = core(&ops[2 * i + 2..]);
                let p = |a, b, op| {
                    match op {
                        &Op('+') => a + b,
                        &Op('-') => a - b,
                        _ => a * b,
                    }
                };
                for &l in lr.iter() {
                    for &r in rr.iter() {
                        result.push(p(l, r, &ops[2 * i + 1]));
                    }
                }
            }
        }
        result
    }

    core(ops.as_slice()).iter().map(|&x| x).collect::<Vec<i32>>()
}