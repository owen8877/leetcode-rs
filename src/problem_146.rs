use std::collections::HashMap;
// TODO: Needs Improvement
struct LRUCache {
    map: HashMap<i32, i32>,
    key_age: Vec<(i32, usize)>,
    capacity: usize,
    timestamp: usize,
}


impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            key_age: Vec::new(),
            capacity: capacity as usize,
            timestamp: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(v) => {
                for i in 0..self.key_age.len() {
                    if self.key_age[i].0 == key {
                        self.key_age[i].1 = self.timestamp;
                        break
                    }
                }
                self.timestamp += 1;
                self.key_age.sort_by_key(|&(_, age)| age);
                *v
            },
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(v) = self.map.get_mut(&key) {
            *v = value;
            for i in 0..self.key_age.len() {
                if self.key_age[i].0 == key {
                    self.key_age[i].1 = self.timestamp;
                    break
                }
            }
            self.timestamp += 1;
            self.key_age.sort_by_key(|&(_, age)| age);
        } else {
            if self.key_age.len() >= self.capacity {
                let (k, _) = self.key_age.remove(0);
                self.map.remove(&k);
            }
            self.key_age.push((key, self.timestamp));
            self.map.insert(key, value);
            self.timestamp += 1;
        }
    }
}