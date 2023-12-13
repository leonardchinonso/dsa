use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// https://leetcode.com/problems/design-add-and-search-words-data-structure/

#[derive(Debug)]
struct TrieNode {
    value: Option<char>,
    is_end: bool,
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
}

impl TrieNode {
    fn new(ch: Option<char>) -> Self {
        Self {
            value: ch,
            is_end: false,
            children: HashMap::new(),
        }
    }

    fn add_child(&mut self, ch: char, node: TrieNode) {
        self.children.insert(ch, Rc::new(RefCell::new(node)));
    }

    fn get_child(&self, ch: &char) -> Option<Rc<RefCell<TrieNode>>> {
        self.children.get(ch).map(|node| Rc::clone(node))
    }

    fn get_children(&self) -> Vec<Rc<RefCell<TrieNode>>> {
        let mut vector = Vec::with_capacity(self.children.len());
        for (k, v) in self.children.iter() {
            vector.push(Rc::clone(v));
        }
        vector
    }
}

struct WordDictionary {
    root: Rc<RefCell<TrieNode>>,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(TrieNode::new(None))),
        }
    }

    fn add_word(&self, word: String) {
        let mut curr_node = self.root.clone();
        for ch in word.chars() {
            let curr_node_clone = curr_node.clone();
            let curr_node_ref = curr_node_clone.borrow();
            let child_node = curr_node_ref.get_child(&ch);
            drop(curr_node_ref);

            if child_node.is_none() {
                let new_node = TrieNode::new(Some(ch));
                let mut curr_node_mut_ref = curr_node_clone.borrow_mut();
                curr_node_mut_ref.add_child(ch, new_node);
                let node = curr_node_mut_ref.get_child(&ch);
                curr_node = node.unwrap();
            } else {
                curr_node = child_node.unwrap();
            }
        }

        let mut curr_node_mut_ref = curr_node.borrow_mut();
        curr_node_mut_ref.is_end = true;
    }

    fn search_helper(&self, word: &[u8], node: Rc<RefCell<TrieNode>>) -> bool {
        let mut curr_node = node;
        for ch in word {
            if *ch as char == '.' {
                let curr_node_clone = curr_node.clone();
                let curr_node_ref = curr_node_clone.borrow();
                let children = curr_node_ref.get_children();

                for child in children {
                    dbg!(word.split_at(1).1.clone());
                    dbg!(child.clone());
                    if self.search_helper(word.split_at(1).1, child) {
                        println!();
                        return true;
                    }
                }

                return false;
            }

            let curr_node_clone = curr_node.clone();
            let curr_node_ref = curr_node_clone.borrow();
            let child_node = curr_node_ref.get_child(&(*ch as char));
            if child_node.is_some() {
                curr_node = child_node.unwrap();
                continue;
            }

            return false;
        }

        let curr_node_ref = curr_node.borrow();
        curr_node_ref.is_end
    }

    fn search(&self, word: String) -> bool {
        self.search_helper(word.as_bytes(), self.root.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_dictionary() {
        let mut dict = WordDictionary::new();
        for word in [
            String::from("bad"),
            String::from("dad"),
            String::from("mad"),
        ] {
            dict.add_word(word);
        }
        for (word, expected) in [
            // (String::from("pad"), false),
            // (String::from("bad"), true),
            // (String::from(".ad"), true),
            (String::from("b.."), true),
        ] {
            let got = dict.search(word);
            assert_eq!(got, expected);
        }
    }
}
