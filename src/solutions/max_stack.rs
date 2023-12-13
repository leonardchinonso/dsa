use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Eq, Hash)]
struct Node {
    id: i32,
    val: i32,
}

impl Node {
    fn new(id: i32, val: i32) -> Self {
        Self { val, id }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.id == other.id
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.val == other.val {
            return self.id.cmp(&other.id);
        }
        return self.val.cmp(&other.val);
    }
}

struct MaxStack {
    stack: Vec<Node>,
    heap: BinaryHeap<Node>,
    deleted: HashSet<Node>,
    next_id: i32,
}

impl MaxStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            heap: BinaryHeap::new(),
            deleted: HashSet::new(),
            next_id: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let node = Node::new(self.next_id, x);
        self.stack.push(node.clone());
        self.heap.push(node);
        self.next_id += 1;
    }

    fn pop(&mut self) -> i32 {
        let mut top = self.stack.pop().unwrap();
        while !self.stack.is_empty() && self.deleted.contains(&top) {
            top = self.stack.pop().unwrap();
        }
        self.deleted.insert(top.clone());
        top.val
    }

    fn top(&mut self) -> i32 {
        let mut top = self.stack.pop().unwrap();
        while !self.stack.is_empty() && self.deleted.contains(&top) {
            top = self.stack.pop().unwrap();
        }
        // push back to the stack since we dont want it out
        self.stack.push(top.clone());
        top.val
    }

    fn peek_max(&mut self) -> i32 {
        let mut top = self.heap.pop().unwrap();
        while !self.heap.is_empty() && self.deleted.contains(&top) {
            top = self.heap.pop().unwrap();
        }
        // push back to the heap since we dont want it out
        self.heap.push(top.clone());
        top.val
    }

    fn pop_max(&mut self) -> i32 {
        let mut top = self.heap.pop().unwrap();
        while !self.heap.is_empty() && self.deleted.contains(&top) {
            top = self.heap.pop().unwrap();
        }
        self.deleted.insert(top.clone());
        top.val
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn max_stack_works() {
        let mut max_stack = MaxStack::new();
        max_stack.push(5);
        // max_stack.push(3);
        max_stack.push(1);
        // max_stack.push(1);
        max_stack.push(5);
        // max_stack.push(5);
        // max_stack.push(5);
        // max_stack.push(5);
        // max_stack.push(5);

        assert_eq!(max_stack.top(), 5);
        assert_eq!(max_stack.pop_max(), 5);
        assert_eq!(max_stack.top(), 1);
        assert_eq!(max_stack.peek_max(), 5);
        assert_eq!(max_stack.pop(), 1);
        assert_eq!(max_stack.top(), 5);
    }
}
