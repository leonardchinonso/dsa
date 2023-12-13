mod heap_impl2 {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    // Node representation of a maxHeap node
    #[derive(Debug, PartialEq, Eq)]
    struct Node {
        letter: char,
        freq: i32,
    }

    // custom methods for the node struct
    impl Node {
        fn new(letter: char, freq: i32) -> Self {
            Self { letter, freq }
        }

        fn dec(&mut self) {
            self.freq -= 1;
        }
    }

    // traits implementation for using Node in the BinaryHeap
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            (self.freq, &self.letter).cmp(&(other.freq, &other.letter))
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut nodes: Vec<Node> = Vec::new();
        if a > 0 {
            nodes.push(Node::new('a', a))
        }
        if b > 0 {
            nodes.push(Node::new('b', b))
        }
        if c > 0 {
            nodes.push(Node::new('c', c))
        }

        // create a maxHeap from the nodes
        let mut heap: BinaryHeap<Node> = BinaryHeap::from(nodes);

        let mut result: Vec<char> = Vec::new();

        while !heap.is_empty() {
            let mut node = heap.pop().unwrap();

            if result.len() >= 2
                && result[result.len() - 2] == node.letter
                && result[result.len() - 1] == node.letter
            {
                // if result has 2 or more elements and the current letter to insert is the same
                // as the last two, pop another node and push the first node back onto the heap
                let second_node = heap.pop();
                // push the first node back into the heap
                heap.push(node);
                // return the result if there are no nodes to process
                if second_node.is_none() {
                    return result.into_iter().collect::<String>();
                }
                // since the second node is valid, add it to the result and decrement the freq
                let mut second_node = second_node.unwrap();
                result.push(second_node.letter);
                second_node.dec();
                // push the second node back into the heap if the freq is not zero
                if second_node.freq > 0 {
                    heap.push(second_node);
                }
            } else {
                // we're free to insert the first node
                result.push(node.letter);
                node.dec();
                // push the first node back into the heap if the freq is not zero
                if node.freq > 0 {
                    heap.push(node);
                }
            }
        }

        return result.into_iter().collect::<String>();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn longest_diverse_string_heap_impl_works() {
        let test_cases = vec![
            (1, 1, 7, 8),
            (7, 1, 0, 5),
            (5, 2, 7, 14),
            (0, 0, 0, 0),
            (0, 8, 11, 19),
        ];

        for test_case in test_cases {
            let got = heap_impl2::longest_diverse_string(test_case.0, test_case.1, test_case.2);
            assert_eq!(got.len(), test_case.3);
        }
    }
}
