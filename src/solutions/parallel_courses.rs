use std::collections::{HashMap, VecDeque};

fn build_graph_and_indegrees(
    n: usize,
    relations: &Vec<Vec<i32>>,
) -> (HashMap<i32, i32>, HashMap<i32, Vec<i32>>) {
    let mut in_degrees = HashMap::with_capacity(n + 1);
    let mut graph = HashMap::with_capacity(n + 1);

    for i in 1..n + 1 {
        let i = i as i32;
        in_degrees.insert(i, 0);
        graph.insert(i, Vec::new());
    }

    for relation in relations {
        let (src, dest) = (relation[0], relation[1]);
        in_degrees.entry(dest).and_modify(|v| *v += 1);
        graph
            .entry(src)
            .and_modify(|nodes: &mut Vec<i32>| nodes.push(dest));
    }

    (in_degrees, graph)
}

fn process_courses(
    n: i32,
    mut in_degrees: HashMap<i32, i32>,
    graph: HashMap<i32, Vec<i32>>,
) -> i32 {
    let mut semesters = 0;
    let mut read_count = 0;

    let mut queue = VecDeque::new();
    for (node, freq) in in_degrees.iter() {
        if *freq == 0 {
            queue.push_back(*node);
        }
    }

    while !queue.is_empty() {
        println!("Queue: {:?}", queue);

        // increment number of semesters, this is a new level
        semesters += 1;

        let mut next_queue = VecDeque::new();

        for node in queue {
            read_count += 1;
            for neighbor in graph.get(&node).unwrap() {
                let freq = in_degrees.get_mut(neighbor).unwrap();
                *freq -= 1;
                if *freq == 0 {
                    next_queue.push_back(*neighbor);
                }
            }
        }

        queue = next_queue;
    }

    if read_count == n {
        semesters
    } else {
        -1
    }
}

pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    let (in_degrees, graph) = build_graph_and_indegrees(n as usize, &relations);
    process_courses(n, in_degrees, graph)
}

#[cfg(test)]
mod test {
    #[test]
    fn minimum_semesters_works() {
        let test_cases = vec![
            (3, vec![vec![1, 3], vec![2, 3]], 2),
            (3, vec![vec![1, 2], vec![2, 3], vec![3, 1]], -1),
        ];

        for test_case in test_cases {
            let got = super::minimum_semesters(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
