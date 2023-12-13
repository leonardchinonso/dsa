use std::collections::HashMap;
// use rand::seq::SliceRandom;

struct RandomizedSet {
    indices_map: HashMap<i32, usize>,
    numbers: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            indices_map: HashMap::new(),
            numbers: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices_map.contains_key(&val) {
            return false;
        }

        self.indices_map.insert(val, self.numbers.len());
        self.numbers.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.indices_map.contains_key(&val) {
            return false;
        }

        let index = self.indices_map.remove(&val).unwrap();
        let last_val = *self.numbers.last().clone().unwrap();
        let n = self.numbers.len();
        if index != n - 1 {
            self.numbers.swap(index, n - 1);
            self.indices_map.insert(last_val, index);
        }
        self.numbers.pop();
        true
    }

    fn get_random(&self) -> i32 {
        // *self.numbers.choose(&mut rand::thread_rng()).unwrap()
        0
    }
}
