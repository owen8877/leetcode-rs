#[derive(Debug)]
struct NestedIterator {
    int_mode: bool,
    int: Option<i32>,
    its: Vec<NestedIterator>,
}

impl NestedIterator {
    fn new_from_int(i: i32) -> Self {
        Self {
            int_mode: true,
            int: Some(i),
            its: vec![],
        }
    }

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut its = vec![];
        for ni in nestedList {
            match ni {
                NestedInteger::Int(i) => its.push(Self::new_from_int(i)),
                NestedInteger::List(v) => its.push(Self::new(v)),
            }
        }
        let mut s = Self {
            int_mode: false,
            int: None,
            its,
        };
        s.reduce();
        s
    }

    fn reduce(&mut self) {
        if self.int_mode {
            return;
        }

        while !self.its.is_empty() {
            if self.its[0].has_next() {
                return;
            }
            self.its.remove(0);
        }
    }

    fn next(&mut self) -> i32 {
        if self.int_mode {
            self.int.take().unwrap()
        } else {
            let result = self.its[0].next();
            self.reduce();
            result
        }
    }

    fn has_next(&self) -> bool {
        if self.int_mode {
            self.int.is_some()
        } else {
            !self.its.is_empty()
        }
    }
}