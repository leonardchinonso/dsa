use std::collections::{HashMap, VecDeque};

pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
    let graph = build_graph(pid, ppid);
    let mut queue = VecDeque::new();
    let mut killed_processes = Vec::new();

    queue.push_back(kill);
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        killed_processes.push(p);
        graph
            .get(&p)
            .iter()
            .for_each(|v| v.iter().for_each(|c| queue.push_back(*c)))
    }

    killed_processes
}

fn build_graph(pid: Vec<i32>, ppid: Vec<i32>) -> HashMap<i32, Vec<i32>> {
    let mut graph = HashMap::with_capacity(ppid.len());

    ppid.iter().enumerate().for_each(|(i, id)| {
        graph
            .entry(ppid[i])
            .and_modify(|v: &mut Vec<i32>| v.push(pid[i]))
            .or_insert(vec![pid[i]]);
    });

    graph
}

#[cfg(test)]
mod test {
    #[test]
    fn kill_process_works() {
        let test_cases = vec![
            (vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5, vec![5, 10]),
            (vec![1], vec![0], 1, vec![1]),
        ];

        for test_case in test_cases {
            let actual = super::kill_process(test_case.0, test_case.1, test_case.2);
            assert_eq!(actual, test_case.3);
        }
    }
}
