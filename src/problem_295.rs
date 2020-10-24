use std::collections::BinaryHeap;

struct MedianFinder {
    hi: BinaryHeap<i32>,
    lo: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            hi: BinaryHeap::new(),
            lo: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.lo.push(num);
        self.hi.push(-self.lo.pop().unwrap());
        if self.hi.len() > self.lo.len() {
            self.lo.push(-self.hi.pop().unwrap());
        }
    }

    fn find_median(&self) -> f64 {
        if self.hi.len() == self.lo.len() {
            (self.lo.peek().unwrap() - self.hi.peek().unwrap()) as f64 / 2.0
        } else {
            *self.lo.peek().unwrap() as f64
        }
    }
}