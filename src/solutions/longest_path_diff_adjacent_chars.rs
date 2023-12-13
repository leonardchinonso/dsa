mod solver {
    use std::cmp::max;

    pub struct Solver {
        pub graph: Vec<Vec<usize>>,
        pub nodes: Vec<char>,
        pub longest_path: i32,
    }

    impl Solver {
        // new creates a graph of the nodes using the parents
        pub fn new(parent: &[i32], nodes: String) -> Self {
            let nodes = nodes.chars().collect::<Vec<char>>();
            let mut graph = vec![Vec::new(); parent.len()];

            for i in 1..parent.len() {
                // push the index of the child in the string into the graph's children vector
                graph[parent[i] as usize].push(i);
            }

            Self {
                graph,
                nodes,
                longest_path: 1,
            }
        }

        // search_depth_first uses dfs to traverse the graph and maintain the invariants
        pub fn search_depth_first(&mut self, curr_node: usize) -> i32 {
            // initialize the longest and second longest paths in the tree from this node
            let (mut longest, mut second_longest) = (0_i32, 0_i32);

            for i in 0..self.graph[curr_node].len() {
                let child_id = self.graph[curr_node][i];

                // get the potential longest path, which is the path of one of its child paths
                let candidate = self.search_depth_first(child_id);

                // move on if the next node is the same as the current node
                // let next_id = self.graph[curr_node][child_id];
                if self.nodes[child_id] == self.nodes[curr_node] {
                    continue;
                }

                // if the candidate does not surpass the second longest, move on
                if candidate < second_longest {
                    continue;
                }

                // if the candidate does not surpass the second longest, move on
                // however, the candidate should be the new second longest
                if candidate < longest {
                    second_longest = candidate;
                    continue;
                }

                // candidate qualifies as the longest so far
                second_longest = longest; // demote longest
                longest = candidate; // promote candidate
            }

            // compute longest path with the current node being in the middle of the path
            self.longest_path = max(self.longest_path, longest + 1 + second_longest);
            longest + 1
        }
    }
}

use solver::Solver;

pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    // create a graph from the parent and node list
    let mut s = Solver::new(&parent, s);

    // using depth-first search, explore from the root node
    s.search_depth_first(0);

    s.longest_path
}

#[cfg(test)]
mod test {
    #[test]
    fn longest_path_works() {
        let test_cases = vec![
            (vec![-1,0,0,1,1,2], "abacbe", 3),
            (vec![-1,0,0,0], "aabc", 3),
            (vec![
                -1,159,58,91,160,65,10,91,9,58,143,66,145,18,143,32,152,108,145,0,154,46,146,148,
                  144,133,58,107,87,144,64,28,46,87,14,14,121,110, 119,152,74,14,14,120,159,9,19,100,
                  9,75,156,127,48,154,140,64,158,8,43,46,27,31,34,74,99,32,45,105,100,32,27,112,9,43,
                  60,8,97,45,2,86,49,35,160,77,111,138,145,152,59,87,9,125,70,20,59,47,91,58,65,43,
                  120,29,86,69,77,15,101,138,7,153,138,56,132,37,122,43,33,62,95,91,46,103,34,63,115,
                  2,132,86,86,9,26,110,8,87,58,30,103,93,43,45,46,143,30,87,97,115,8,58,125,10,152,
                  10,2,107,141,108,37,32,43,69,100
            ], "ofarlvkejasusszlxapxqmpecaduhuogzltxuclafjrbrvqtsidmmrjjespdupikvyopweptnjteylnixcjj\
            fsubmhcekgbdorwihftfbtrqhzinccijwbpjaafkdplnjrydmkrluvdcmmoohwgkdmeuuqtsktbyl", 18)
        ];

        for test_case in test_cases {
            let got = super::longest_path(test_case.0, test_case.1.to_string());
            assert_eq!(got, test_case.2);
        }
    }
}
