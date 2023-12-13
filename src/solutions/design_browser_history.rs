// use std::cell::RefCell;
// use std::rc::Rc;
//
// /// https://leetcode.com/problems/design-browser-history/description/
// /// company: bloomberg
//
// type Link = Option<Rc<RefCell<Node>>>;
//
// struct Node {
//     url: String,
//     next: Link,
//     prev: Link,
// }
//
// impl Node {
//     fn new(url: String) -> Self {
//         Self {
//             url,
//             prev: None,
//             next: None,
//         }
//     }
// }
//
// struct DoublyLinkedList {
//     head: Rc<RefCell<Node>>,
//     tail: Rc<RefCell<Node>>,
//     ptr: Rc<RefCell<Node>>,
// }
//
// impl DoublyLinkedList {
//     fn new() -> Self {
//         let head = Rc::new(RefCell::new(Node::new("#".to_string())));
//         let tail = Rc::new(RefCell::new(Node::new("#".to_string())));
//
//         Self {
//             head: Rc::clone(&head),
//             tail,
//             ptr: head
//         }
//     }
//
//     fn insert(&self, url: String) {
//         let node = Rc::new(RefCell::new(Node::new(url)));
//         node.borrow_mut().next = Some(Rc::clone(&self.tail));
//         // self.tail.borrow_mut().prev = Some(Rc::clone(&node));
//     }
// }
//
//
// struct BrowserHistory {
//     list: DoublyLinkedList,
// }
//
// impl BrowserHistory {
//
//     fn new(homepage: String) -> Self {
//         let list = DoublyLinkedList::new();
//         list.insert(homepage);
//         Self{
//             list,
//         }
//     }
//
//     fn visit(&self, url: String) {
//         self.list.insert(url);
//     }
//
//     fn back(&self, steps: i32) -> String {
//         let mut steps = steps;
//         let mut curr = &self.list.ptr;
//         while steps > 0 && curr.borrow().prev != &self.list.head {
//             curr = &curr.borrow().prev;
//             steps -= 1;
//         }
//         String::new()
//     }
//
//     fn forward(&self, steps: i32) -> String {
//         String::new()
//     }
// }
//
// #[cfg(test)]
// mod test {
//     #[test]
//     fn it_compiles() {}
// }
