struct MyQueue {
    stack: Vec<i32>,
    pop_mode: bool,
}

impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: vec![],
            pop_mode: false,
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        if self.pop_mode {
            self.flip();
        }
        self.stack.push(x)
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if !self.pop_mode {
            self.flip();
        }
        self.stack.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if !self.pop_mode {
            self.flip();
        }
        *self.stack.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn flip(&mut self) {
        self.pop_mode = !self.pop_mode;
        let mut s = vec![];
        while let Some(x) = self.stack.pop() {
            s.push(x);
        }
        self.stack = s;
    }
}