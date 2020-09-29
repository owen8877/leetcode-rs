use std::cmp::*;

struct MinStack {
    stack: Vec<i32>,
    m: i32,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            m: 0,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() == 0 {
            self.m = x;
        } else {
            self.m = min(self.m, x);
        }
        self.stack.push(x);
    }

    fn pop(&mut self) {
        let x = self.stack.pop().unwrap();
        if x <= self.m {
            if self.stack.len() > 0 {
                let mut mm = self.stack[0];
                for i in 1..self.stack.len() {
                    mm = min(mm, self.stack[i]);
                }
                self.m = mm;
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.m
    }
}