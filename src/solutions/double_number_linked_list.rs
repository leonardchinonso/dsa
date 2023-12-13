// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // 9 -> 9 -> 9 ... before reverse
    let head = reverse(head);
    // 9 -> 9 -> 9 ... after reverse

    // 9 -> 9 -> 9 ... before double
    let head = double_each(head);
    // 8 -> 9 -> 9 -> 1 ... after double

    print_list(head.clone());

    return reverse(head);
}

fn double_each(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut curr = head.as_mut().unwrap();
    let mut carry = 0;
    loop {
        let val = curr.val * 2;
        curr.val = (val % 10) + carry;
        carry = val / 10;
        if curr.next.is_none() {
            break;
        }
        curr = curr.next.as_mut().unwrap();
    }
    if carry != 0 {
        let node = Box::new(ListNode::new(carry));
        curr.next = Some(node);
    }
    head
}

// reverse reverses a linked list in-place
fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;
    while curr.is_some() {
        let mut curr_node = curr.unwrap();
        let next = curr_node.next;
        curr_node.next = prev;
        prev = Some(curr_node);
        curr = next;
    }
    prev
}

fn print_list(head: Option<Box<ListNode>>) {
    let mut curr = head.clone();
    while curr.is_some() {
        curr = curr.unwrap().next;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_linked_list(vector: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));
        let mut curr = &mut dummy_head;
        for num in vector {
            let node = Some(Box::new(ListNode::new(*num)));
            if let Some(temp) = curr {
                temp.next = node;
                curr = &mut temp.next;
            }
        }
        dummy_head.unwrap().next
    }

    fn compare_linked_list_to_vector(head: Option<Box<ListNode>>, vector: &[i32]) -> bool {
        let mut curr = head;
        let mut i = 0_usize;
        while curr.is_some() && i < vector.len() {
            let curr_node = curr.unwrap();
            if curr_node.val != vector[i] {
                return false;
            }
            curr = curr_node.next;
            i += 1;
        }
        return i == vector.len() && curr.is_none();
    }

    #[test]
    fn reverse_works() {
        let test_cases = vec![
            (vec![1, 8, 9], vec![9, 8, 1]),
            (vec![9, 9, 9], vec![9, 9, 9]),
        ];

        for test_case in test_cases {
            let head = create_linked_list(&test_case.0);
            let head = reverse(head);
            assert!(compare_linked_list_to_vector(head.clone(), &test_case.1))
        }
    }

    #[test]
    fn double_it_works() {
        let test_cases = vec![
            (vec![1, 8, 9], vec![3, 7, 8]),
            (vec![9, 9, 9], vec![1, 9, 9, 8]),
        ];

        for test_case in test_cases {
            let head = create_linked_list(&test_case.0);
            assert!(compare_linked_list_to_vector(head.clone(), &test_case.0));

            let head = double_it(head);
            assert!(compare_linked_list_to_vector(head.clone(), &test_case.1));
        }
    }
}
