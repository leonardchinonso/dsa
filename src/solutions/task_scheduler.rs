pub mod heap_impl {
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashMap};
    use std::fmt::Debug;

    #[derive(Debug, PartialEq, Eq)]
    struct Node {
        task: char,
        freq: i32,
        last_pos: i32,
    }

    impl Node {
        pub fn new(task: char, freq: i32) -> Self {
            Self {
                task,
                freq,
                last_pos: -1,
            }
        }

        pub fn dec(&mut self) {
            self.freq -= 1;
        }

        pub fn set_last_pos(&mut self, last_pos: i32) {
            self.last_pos = last_pos;
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(&other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            (self.freq, self.task).cmp(&(other.freq, other.task))
        }
    }

    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n == 0 {
            return tasks.len() as i32;
        }

        let mut hashmap: HashMap<char, Node> = HashMap::new();
        for task in tasks {
            hashmap
                .entry(task)
                .and_modify(|node| node.freq += 1)
                .or_insert(Node::new(task, 1));
        }

        let mut heap: BinaryHeap<RefCell<Node>> = BinaryHeap::new();
        for (_, node) in hashmap.into_iter() {
            heap.push(RefCell::new(node));
        }

        let mut stack: Vec<RefCell<Node>> = Vec::with_capacity(heap.len());
        let mut ptr = 0i32;
        while !heap.is_empty() {
            let mut node_ref = heap.pop().unwrap();

            if node_ref.borrow().last_pos == -1 || ptr - node_ref.borrow().last_pos > n {
                node_ref.borrow_mut().dec();
                node_ref.borrow_mut().set_last_pos(ptr);
                ptr += 1;
                if node_ref.borrow().freq > 0 {
                    heap.push(node_ref);
                }
                continue;
            }

            stack.push(node_ref);
            let mut found = false;
            while !heap.is_empty() {
                node_ref = heap.pop().unwrap();
                if node_ref.borrow().last_pos == -1 || ptr - node_ref.borrow().last_pos > n {
                    found = true;
                    node_ref.borrow_mut().dec();
                    node_ref.borrow_mut().set_last_pos(ptr);
                    ptr += 1;
                    if node_ref.borrow().freq > 0 {
                        heap.push(node_ref);
                    }
                    break;
                }
                stack.push(node_ref);
            }

            if !found {
                ptr += 1;
            }

            while !stack.is_empty() {
                heap.push(stack.pop().unwrap());
            }
        }

        ptr
    }
}

pub mod math_impl {
    use std::cmp::max;

    // max( (f - 1) * (n + 1) + X , totalJobs)
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut frequencies: [i32; 26] = [0; 26];
        let tasks_len = tasks.len();
        for task in tasks {
            frequencies[(task as u32 - 'A' as u32) as usize] += 1;
        }

        let max_freq = frequencies.iter().max().unwrap();
        let max_num_count = frequencies.iter().filter(|num| *num == max_freq).count();

        return max(
            tasks_len as i32,
            (max_freq - 1) * (n + 1) + max_num_count as i32,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn least_interval_heap_impl_works() {
        let test_cases = vec![
            (vec!['A', 'A', 'A', 'B', 'B', 'B'], 2, 8),
            (vec!['A', 'A', 'A', 'B', 'B', 'B'], 0, 6),
            (
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2,
                16,
            ),
            (
                vec![
                    'A', 'A', 'A', 'B', 'B', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
                ],
                7,
                18,
            ),
        ];

        for test_case in test_cases {
            let got = heap_impl::least_interval(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }

    #[test]
    fn least_interval_math_impl_works() {
        let test_cases = vec![
            (vec!['A', 'A', 'A', 'B', 'B', 'B'], 2, 8),
            (vec!['A', 'A', 'A', 'B', 'B', 'B'], 0, 6),
            (
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2,
                16,
            ),
            (
                vec![
                    'A', 'A', 'A', 'B', 'B', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
                ],
                7,
                18,
            ),
        ];

        for test_case in test_cases {
            let got = math_impl::least_interval(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
