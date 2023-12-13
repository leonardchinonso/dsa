/// https://leetcode.com/problems/min-stack
/// company: bloomberg

struct MinNode {
    own_value: i32,
    min_value: i32,
}

impl MinNode {
    fn new(val: i32) -> Self {
        Self {
            own_value: val,
            min_value: val,
        }
    }

    fn set_min_value(&mut self, val: i32) {
        self.min_value = val;
    }
}

struct MinStack {
    data: Vec<MinNode>,
}

impl MinStack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn peek(&self) -> Option<&MinNode> {
        self.data.last()
    }

    fn push(&mut self, val: i32) {
        let mut min_node = MinNode::new(val);
        if !self.is_empty() {
            let top = self.peek().unwrap();
            min_node.set_min_value(std::cmp::min(top.min_value, val));
        }
        self.data.push(min_node);
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        let popped = self.peek().unwrap();
        popped.own_value
    }

    fn get_min(&self) -> i32 {
        let popped = self.peek().unwrap();
        popped.min_value
    }
}
