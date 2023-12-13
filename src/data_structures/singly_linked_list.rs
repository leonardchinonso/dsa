// use std::cell::RefCell;
// use std::rc::Rc;
//
// type Link<T> = Option<Rc<RefCell<Node<T>>>>;
//
// pub struct Node<T> {
//     value: T,
//     prev: Link<T>,
//     next: Link<T>,
// }
//
// impl<T> Node<T> {
//     // new returns a new node object
//     pub fn new(value: T) -> Self {
//         Self {
//             value,
//             prev: None,
//             next: None,
//         }
//     }
// }
//
// pub struct LinkedList<T> {
//     head: Node<T>,
//     tail: Node<T>,
//     size: usize,
// }
//
// // impl<T> LinkedList<T> {
// //     // value: T
// // }
