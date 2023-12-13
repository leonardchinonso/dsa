use std::cmp::Ordering;
/// https://leetcode.com/problems/sort-integers-by-the-power-value/
/// company: bloomberg
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq)]
struct Node {
    num: i32,
    pow: i32,
}

impl Node {
    fn new(num: i32, pow: i32) -> Self {
        Self { num, pow }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pow == other.pow && self.num == other.num
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pow == other.pow {
            return self.num.cmp(&other.num);
        }
        return self.pow.cmp(&other.pow);
    }
}

fn get_power(num: i32, mut hashmap: &mut HashMap<i32, i32>) -> i32 {
    if num == 1 {
        return 0;
    }

    if hashmap.contains_key(&num) {
        let res = hashmap.get(&num).unwrap();
        return *res;
    }

    let mut next_num = 0;
    if num % 2 == 0 {
        next_num = num / 2;
    } else {
        next_num = 3 * num + 1;
    }

    let res = 1 + get_power(next_num, hashmap);
    hashmap.insert(num, res);

    res
}

pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let k = k as usize;
    let mut heap = BinaryHeap::with_capacity(k);
    let mut hashmap = HashMap::with_capacity((hi - lo) as usize + 1);
    for num in lo..hi + 1 {
        let pow = get_power(num, &mut hashmap);
        let node = Node::new(num, pow);

        if heap.len() < k {
            heap.push(node);
            continue;
        }

        let top = heap.peek().unwrap();
        if top.pow > node.pow {
            heap.pop(); // remove the top
            heap.push(node);
        } else if top.pow == node.pow && top.num > node.num {
            heap.pop(); // remove the top
            heap.push(node);
        }
    }

    heap.pop().unwrap().num
}

#[cfg(test)]
mod test {
    #[test]
    fn get_kth_works() {
        let test_cases = vec![(12, 15, 2, 13), (7, 11, 4, 7)];

        for test_case in test_cases {
            let actual = super::get_kth(test_case.0, test_case.1, test_case.2);
            assert_eq!(actual, test_case.3);
        }
    }
}
