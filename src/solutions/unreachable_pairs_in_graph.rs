/// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/editorial/
use std::collections::HashMap;

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let graph = edges.into_iter().fold(HashMap::new(), |mut hashmap, edge| {
        hashmap.entry(edge[0]).or_insert(vec![]).push(edge[1]);
        hashmap.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        hashmap
    });

    let mut pair_count = 0i64;
    let mut component_size = 0i64;
    let mut unvisited_nodes = n as i64;
    let mut visited = vec![false; n as usize];

    for node in 0..n {
        if !visited[node as usize] {
            component_size = traverse_component(&graph, node, &mut visited);
            pair_count += component_size * (unvisited_nodes - component_size);
            unvisited_nodes -= component_size;
        }
    }

    pair_count
}

fn traverse_component(
    graph: &HashMap<i32, Vec<i32>>,
    curr_node: i32,
    visited: &mut Vec<bool>,
) -> i64 {
    // mark the node as visited
    visited[curr_node as usize] = true;

    let mut node_count = 1i64;
    let neighbors = match graph.get(&curr_node) {
        Some(neighbors) => neighbors,
        None => return node_count,
    };

    for neighbor in neighbors {
        let neighbor_copy = *neighbor;
        if !visited[neighbor_copy as usize] {
            node_count += traverse_component(graph, neighbor_copy, visited);
        }
    }

    node_count
}

#[cfg(test)]
mod test {
    #[test]
    fn count_pairs_works() {
        let test_cases = vec![
            (3, vec![vec![0, 1], vec![0, 2], vec![1, 2]], 0),
            (
                7,
                vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]],
                14,
            ),
        ];

        for test_case in test_cases {
            let got = super::count_pairs(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
