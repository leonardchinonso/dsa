use std::collections::VecDeque;

/// https://leetcode.com/problems/design-most-recently-used-queue/
/// company: bloomberg

struct MRUQueue {
    buckets: Vec<VecDeque<i32>>,
    n_sqrt: usize,
}

impl MRUQueue {
    fn new(n: i32) -> Self {
        let n = n as usize;
        let n_sqrt = (n as f64).sqrt().ceil() as usize;
        let mut buckets = Vec::with_capacity(n_sqrt);
        let mut queue = VecDeque::with_capacity(n_sqrt);
        for i in 1..n + 1 {
            queue.push_back(i as i32);
            if i % n_sqrt == 0 {
                buckets.push(queue);
                queue = VecDeque::new();
            }
        }

        if !queue.is_empty() {
            buckets.push(queue);
        }

        Self { buckets, n_sqrt }
    }

    fn fetch(&mut self, k: i32) -> i32 {
        let k = k as usize - 1;
        let bucket_idx = k / self.n_sqrt;
        let bucket = self.buckets[bucket_idx].clone();
        let val_idx = k % self.n_sqrt;
        let val_to_move = bucket[val_idx];

        // remove the value by filtering it out
        self.buckets[bucket_idx] = bucket
            .into_iter()
            .filter(|v| *v != val_to_move)
            .collect::<VecDeque<i32>>();

        // push the value to remove to the end of the last bucket queue
        let last = self.buckets.last_mut().unwrap();
        last.push_back(val_to_move);

        // move all the elements from the removed position one to the left
        for i in bucket_idx..self.buckets.len() - 1 {
            let v = self.buckets[i + 1].pop_front().unwrap();
            self.buckets[i].push_back(v);
        }

        val_to_move
    }
}
