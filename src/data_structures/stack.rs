pub struct Stack<T> {
    store: Vec<T>,
    cap: usize,
}

impl<T> Stack<T> {
    // new creates a new stack and returns it
    pub fn new(cap: usize) -> Self {
        Self {
            store: Vec::with_capacity(cap),
            cap,
        }
    }
}

impl<T> Stack<T> {
    // push pushes an item to the stack
    pub fn push(&mut self, item: T) {
        self.store.push(item);
    }

    // pop pops an item from the stack
    // returns None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        self.store.pop()
    }

    // is_empty checks that a stack is empty
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    // len returns the length of the stack
    pub fn len(&self) -> usize {
        self.store.len()
    }

    // peek shows what is on top of the stack without returning it
    pub fn peek(&self) -> Option<&T> {
        self.store.last()
    }
}

#[cfg(test)]
mod test {
    use crate::data_structures::stack::Stack;

    #[test]
    fn new_works() {
        let test_cases = vec![(2, 2), (0, 0), (1, 1)];
        for test_case in test_cases {
            let stack: Stack<i32> = Stack::new(test_case.0);
            assert_eq!(stack.cap, test_case.1)
        }
    }

    #[test]
    fn push_works() {
        let test_cases = vec![2, 0, 1];
        for test_case in test_cases {
            let mut stack = Stack::new(test_case);
            assert_eq!(stack.len(), 0);
            for i in 0..test_case {
                stack.push(i)
            }
            assert_eq!(stack.len(), test_case)
        }
    }

    #[test]
    fn pop_works() {
        let test_cases: Vec<(usize, Option<usize>)> = vec![(2, Some(1)), (0, None), (1, Some(0))];
        for test_case in test_cases {
            let mut stack = Stack::new(test_case.0);
            for i in 0..test_case.0 {
                stack.push(i)
            }
            assert_eq!(stack.pop(), test_case.1)
        }
    }

    #[test]
    fn is_empty_works() {
        let test_cases = vec![(2, false), (0, true), (1, false)];
        for test_case in test_cases {
            let mut stack = Stack::new(test_case.0);
            for i in 0..test_case.0 {
                stack.push(i)
            }
            assert_eq!(stack.is_empty(), test_case.1)
        }
    }

    #[test]
    fn peek_works() {
        let test_cases: Vec<(usize, Option<&usize>)> =
            vec![(2, Some(&1)), (0, None), (1, Some(&0))];
        for test_case in test_cases {
            let mut stack = Stack::new(test_case.0);
            for i in 0..test_case.0 {
                stack.push(i)
            }
            assert_eq!(stack.peek(), test_case.1)
        }
    }
}
