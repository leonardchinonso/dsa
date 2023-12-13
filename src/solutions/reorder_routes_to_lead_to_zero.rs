/// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero

#[derive(Debug, Clone)]
enum Edge {
    Actual,
    Fake,
}

impl Edge {
    fn inc_flip(&self) -> i32 {
        match self {
            Edge::Actual => 1,
            Edge::Fake => 0,
        }
    }
}

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut to_flip = 0_i32;
    let mut mapping: Vec<Vec<(i32, Edge)>> = vec![vec![]; n as usize];

    for connection in connections {
        let (src, dest) = (connection[0], connection[1]);
        mapping[src as usize].push((dest, Edge::Actual));
        mapping[dest as usize].push((src, Edge::Fake));
    }

    dfs(0, -1, &mapping, &mut to_flip);

    to_flip
}

fn dfs(node: i32, parent: i32, graph: &Vec<Vec<(i32, Edge)>>, count: &mut i32) {
    for (child, edge_type) in graph[node as usize].iter() {
        if *child != parent {
            let flip_val = edge_type.inc_flip();
            *count += flip_val;
            dfs(*child, node, graph, count);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn min_reorder_works() {
        let test_cases = vec![
            (
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
                3,
            ),
            (5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]], 2),
            (3, vec![vec![1, 0], vec![2, 0]], 0),
        ];

        for test_case in test_cases {
            let got = super::min_reorder(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
