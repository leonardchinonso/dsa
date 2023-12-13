use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    letter: char,
    freq: i32,
}

impl Node {
    fn new(letter: char, freq: i32) -> Self {
        Self { letter, freq }
    }

    fn dec(&mut self) {
        self.freq -= 1;
    }

    fn inc(&mut self) {
        self.freq += 1;
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.freq, self.letter).cmp(&(other.freq, other.letter))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn reorganize_string(s: String) -> String {
    let mut result: Vec<char> = Vec::new();
    let mut hashmap: HashMap<char, Node> = HashMap::new();
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    for ch in s.chars() {
        hashmap
            .entry(ch)
            .and_modify(|node| node.inc())
            .or_insert(Node::new(ch, 1));
    }

    for (_, node) in hashmap {
        heap.push(node);
    }

    while let Some(mut node) = heap.pop() {
        if result.is_empty() || *result.last().unwrap() != node.letter {
            result.push(node.letter);
            node.dec();
            if node.freq > 0 {
                heap.push(node);
            }
        } else {
            let second_node = heap.pop();
            if second_node.is_none() {
                return String::new();
            }
            let mut second_node = second_node.unwrap();
            heap.push(node);
            result.push(second_node.letter);
            second_node.dec();
            if second_node.freq > 0 {
                heap.push(second_node);
            }
        }
    }

    result.into_iter().collect::<String>()
}

#[cfg(test)]
mod test {
    #[test]
    fn reorganize_string_works() {
        let test_cases = vec![("aab", "aba"), ("aaab", "")];

        for test_case in test_cases {
            let got = super::reorganize_string(test_case.0.to_string());
            assert_eq!(got.as_str(), test_case.1);
        }
    }
}
