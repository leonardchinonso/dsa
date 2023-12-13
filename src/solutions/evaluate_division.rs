use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// https://leetcode.com/problems/evaluate-division/

struct Node {
    name: String,
    edges: Vec<Rc<RefCell<Edge>>>
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut edges = Vec::with_capacity(self.edges.len());
        self.edges.iter().for_each(|e| {
            let curr_e = Rc::clone(&e);
            let curr_e_ref = curr_e.borrow();
            let next_node_ref = curr_e_ref.node.borrow();
            edges.push(format!("{}->{}", next_node_ref.name.clone(), curr_e_ref.multiplier));
        });
        write!(f, "{} edges: {:?}", self.name, edges)
    }
}

impl Node {
    fn new(name: String) -> Self {
        Self {
            name,
            edges: Vec::new()
        }
    }

    fn add_edge(&mut self, multiplier: f64, other_node: Rc<RefCell<Node>>) {
        let edge = Edge::new(multiplier, other_node);
        self.edges.push(Rc::new(RefCell::new(edge)));
    }
}

struct Edge {
    multiplier: f64,
    node: Rc<RefCell<Node>>
}

impl Edge {
    fn new(multiplier: f64, node: Rc<RefCell<Node>>) -> Self {
        Self {
            multiplier,
            node
        }
    }
}

fn build_graph(equations: &Vec<Vec<String>>, values: &Vec<f64>,) -> HashMap<String, Rc<RefCell<Node>>> {
    let mut hashmap: HashMap<String, Rc<RefCell<Node>>> = equations.iter().enumerate().fold(HashMap::new(), |mut h, (idx, eqn)| {
        let (start_string, end_string) = (eqn[0].clone(), eqn[1].clone());

        // get the start_node
        let start_node = match h.get(&start_string) {
            Some(start_node) => Rc::clone(start_node),
            None => {
                let node = Rc::new(RefCell::new(Node::new(start_string.clone())));
                h.insert(start_string, Rc::clone(&node));
                node
            }
        };
        let end_node = match h.get(&end_string) {
            Some(end_node) => Rc::clone(end_node),
            None => {
                let node = Rc::new(RefCell::new(Node::new(end_string.clone())));
                h.insert(end_string, Rc::clone(&node));
                node
            }
        };

        start_node.borrow_mut().add_edge(values[idx], Rc::clone(&end_node));
        end_node.borrow_mut().add_edge(1f64/values[idx], Rc::clone(&start_node));

        h
    });

    hashmap
}

#[derive(Debug)]
struct QueueNode(Rc<RefCell<Node>>, f64);

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut results = vec![0f64; queries.len()];
    let names_to_nodes = build_graph(&equations, &values);

    for (idx, query) in queries.iter().enumerate() {
        let mut visited = HashSet::new();
        let (start_string, end_string) = (query.first().unwrap(), query.last().unwrap());

        // check that the query nodes are valid query nodes
        if !names_to_nodes.contains_key(start_string) || !names_to_nodes.contains_key(end_string) {
            results[idx] = -1f64;
            continue;
        }

        let start_node = names_to_nodes.get(start_string).unwrap().clone();
        let end_node = names_to_nodes.get(end_string).unwrap().clone();
        let mut queue: VecDeque<QueueNode> = VecDeque::new();
        queue.push_back(QueueNode(Rc::clone(&start_node), 1f64));
        visited.insert(start_string.clone());

        while !queue.is_empty() {
            let curr_node = queue.pop_front().unwrap();
            let (curr_node, curr_amount) = (curr_node.0, curr_node.1);

            if curr_node.borrow().name == end_node.borrow().name {
                results[idx] = curr_amount;
                break;
            }

            // for each edge of curr_node, multiply curr_amount by the next node's amount and push
            // it back to the queue. Do this ONLY IF the node has not been visited
            for edge in curr_node.borrow().edges.iter() {
                let edge_ref = edge.borrow();
                if !visited.contains(&edge_ref.node.borrow().name.clone()) {
                    queue.push_back(QueueNode(Rc::clone(&edge_ref.node), edge_ref.multiplier * curr_amount));
                    visited.insert(edge_ref.node.borrow().name.clone());
                }
            }
        }

        if results[idx] == 0f64 {
            results[idx] = -1f64;
        }
    }

    results
}

#[cfg(test)]
mod test {
    #[test]
    fn calc_equation_works() {
        let test_cases = vec![
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                ],
                vec![2f64, 3f64],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("e")],
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("x"), String::from("x")],
                ],
                vec![6f64, 0.5f64, -1f64, 1f64, -1f64],
            ),
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                    vec![String::from("bc"), String::from("cd")],
                ],
                vec![1.5f64, 2.5f64, 5f64],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("c"), String::from("b")],
                    vec![String::from("bc"), String::from("cd")],
                    vec![String::from("cd"), String::from("bc")],
                ],
                vec![3.75f64, 0.4f64, 5f64, 0.2f64],
            ),
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("c"), String::from("d")],
                ],
                vec![1f64, 1f64],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("d")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("d"), String::from("c")],
                ],
                vec![-1f64, -1f64, 1f64, 1f64],
            ),
        ];

        for test_case in test_cases {
            let actual = super::calc_equation(test_case.0, test_case.1, test_case.2);
            assert_eq!(actual, test_case.3);
        }
    }
}
