use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct AirStrip {
    graph: HashMap<String, HashSet<String>>,
}

impl AirStrip {
    fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    fn add_path(&mut self, src: &str, dest: &str) {
        let (src, dest) = (src.to_string(), dest.to_string());
        self.graph
            .entry(src)
            .and_modify(|dests| {
                dests.insert(dest.clone());
            })
            .or_insert({
                let mut h = HashSet::new();
                h.insert(dest);
                h
            });
    }

    fn all_paths_between_nodes(&self, src: &str, dest: &str) -> Vec<String> {
        let (src, dest) = (src.to_string(), dest.to_string());
        let mut paths = Vec::new();
        let mut curr_path = src.clone();
        self.dfs(src, &dest, curr_path, &mut paths);
        paths
    }

    fn dfs(&self, start: String, end: &String, mut curr_path: String, mut paths: &mut Vec<String>) {
        // base case: if the start is the same as the end
        if start == *end {
            paths.push(curr_path);
            return;
        }

        let children = self.graph.get(&start);
        if children.is_none() {
            return;
        }

        for child in children.unwrap() {
            let mut new_curr_path = curr_path.clone();
            let c = child.clone();
            new_curr_path.push_str(" -> ");
            new_curr_path.push_str(c.as_str());
            self.dfs(c, end, new_curr_path, paths);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::air_strip::AirStrip;

    #[test]
    fn all_paths_between_nodes_works() {
        let mut air_strip = AirStrip::new();
        air_strip.add_path("A", "B");
        air_strip.add_path("B", "C");
        air_strip.add_path("B", "C");
        air_strip.add_path("A", "C");
        air_strip.add_path("A", "D");
        air_strip.add_path("D", "C");
        air_strip.add_path("F", "E");
        air_strip.add_path("A", "F");

        dbg!(air_strip.clone());

        let paths = air_strip.all_paths_between_nodes("A", "C");
        dbg!(paths);
    }
}
