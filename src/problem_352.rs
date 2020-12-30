struct SummaryRanges {
    summary: Vec<Vec<i32>>,
}

impl SummaryRanges {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            summary: vec![],
        }
    }

    fn add_num(&mut self, val: i32) {
        if self.summary.is_empty() {
            self.summary.push(vec![val, val]);
        } else {
            if val < self.summary[0][0] - 1 {
                self.summary.insert(0, vec![val, val]);
            } else if val == self.summary[0][0] - 1 {
                self.summary[0][0] = val;
            } else if val > self.summary.last().unwrap()[1] + 1 {
                self.summary.push(vec![val, val]);
            } else if val == self.summary.last().unwrap()[1] + 1 {
                let n = self.summary.len();
                self.summary[n - 1][1] = val;
            } else {
                let mut left = 0;
                let mut right = self.summary.len();

                if right == 1 {
                    return;
                }
                while right - left > 1 {
                    let mid = (left + right) / 2;
                    if self.summary[mid][0] <= val && val <= self.summary[mid][1] {
                        return;
                    }
                    if self.summary[mid][0] > val {
                        right = mid;
                    } else {
                        left = mid;
                    }
                }

                if self.summary[left][1] < val - 1 {
                    if self.summary[left + 1][0] > val + 1 {
                        self.summary.insert(left + 1, vec![val, val]);
                    } else {
                        self.summary[left + 1][0] = val;
                    }
                } else if self.summary[left][1] == val - 1 {
                    if self.summary[left + 1][0] > val + 1 {
                        self.summary[left][1] = val;
                    } else {
                        let l = self.summary[left][0];
                        let r = self.summary[left + 1][1];
                        self.summary.remove(left);
                        self.summary[left] = vec![l, r];
                    }
                }
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.summary.clone()
    }
}