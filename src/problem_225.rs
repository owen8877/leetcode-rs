use std::collections::VecDeque;

struct MyStack {
    on1: bool,
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            on1: true,
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        if self.on1 {
            self.q1.push_back(x);
        } else {
            self.q2.push_back(x);
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.on1 {
            self.on1 = false;
            while let Some(x) = self.q1.pop_front() {
                if self.q1.len() == 0 {
                    return x;
                } else {
                    self.q2.push_back(x);
                }
            }
        } else {
            self.on1 = true;
            while let Some(x) = self.q2.pop_front() {
                if self.q2.len() == 0 {
                    return x;
                } else {
                    self.q1.push_back(x);
                }
            }
        }
        0
    }

    /** Get the top element. */
    fn top(&mut self) -> i32 {
        if self.on1 {
            self.on1 = false;
            while let Some(x) = self.q1.pop_front() {
                self.q2.push_back(x);
                if self.q1.len() == 0 {
                    return x;
                }
            }
        } else {
            self.on1 = true;
            while let Some(x) = self.q2.pop_front() {
                self.q1.push_back(x);
                if self.q2.len() == 0 {
                    return x;
                }
            }
        }
        0
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        if self.on1 {
            self.q1.is_empty()
        } else {
            self.q2.is_empty()
        }
    }
}