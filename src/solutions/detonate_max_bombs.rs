use std::cmp::max;
use std::collections::HashSet;

/// https://leetcode.com/problems/detonate-the-maximum-bombs/
/// company: google

pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let graph = build_graph(&bombs);
    return traverse(&graph, &bombs);
}

fn build_graph(bombs: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; bombs.len()];
    for (i, bomb1) in bombs.iter().enumerate() {
        let r_square = bomb1[2].pow(2);
        for (j, bomb2) in bombs.iter().enumerate() {
            let dist = calculate_euclidean_distance(bomb1, bomb2);
            if r_square >= dist {
                graph[i].push(j);
            }
        }
    }
    graph
}

fn calculate_euclidean_distance(bomb1: &Vec<i32>, bomb2: &Vec<i32>) -> i32 {
    let (x1, y1) = (bomb1[0], bomb1[1]);
    let (x2, y2) = (bomb2[0], bomb2[1]);
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}

fn traverse(graph: &Vec<Vec<usize>>, bombs: &Vec<Vec<i32>>) -> i32 {
    let mut max_exploded = 0;
    for node in 0..graph.len() {
        let mut visited = HashSet::with_capacity(bombs.len());
        dfs(node, &graph, &mut visited);
        let exploded = visited.len();
        max_exploded = max(max_exploded, exploded as i32);
    }
    max_exploded
}

fn dfs(node: usize, graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>) {
    // check if bomb has been triggered
    if visited.contains(&node) {
        return;
    }

    // trigger the bomb
    visited.insert(node);

    // check all the bombs that can be reached from this one
    for child in graph[node].iter() {
        dfs(*child, graph, visited);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn maximum_detonation_works() {
        let test_cases = vec![
            (vec![vec![2, 1, 3], vec![6, 1, 4]], 2),
            (vec![vec![1, 1, 5], vec![10, 10, 5]], 1),
            (
                vec![
                    vec![1, 2, 3],
                    vec![2, 3, 1],
                    vec![3, 4, 2],
                    vec![4, 5, 3],
                    vec![5, 6, 4],
                ],
                5,
            ),
        ];

        for test_case in test_cases {
            let got = super::maximum_detonation(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
