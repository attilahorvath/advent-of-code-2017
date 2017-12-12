use std::collections::{HashMap, HashSet};

pub struct Graph {
    nodes: HashMap<u32, Node>,
}

struct Node {
    neighbours: HashSet<u32>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    pub fn parse_node(&mut self, s: &str) {
        let mut iter = s.split_whitespace();

        let index = iter.next()
            .expect("invalid node definition")
            .parse()
            .expect("invalid node index");

        iter.next();

        let mut neighbours = Vec::new();

        while let Some(i) = iter.next() {
            let n = i.trim_matches(',').parse().expect("invalid neighbour data");
            neighbours.push(n);
        }

        self.add_node(index, &neighbours);
    }

    pub fn add_node(&mut self, index: u32, neighbours: &[u32]) {
        for &n in neighbours {
            self.link_nodes(index, n);
        }
    }

    fn link_nodes(&mut self, a: u32, b: u32) {
        {
            let node = self.nodes.entry(a).or_insert(Node::new());

            if a != b {
                (*node).neighbours.insert(b);
            }
        }

        {
            let node = self.nodes.entry(b).or_insert(Node::new());

            if a != b {
                (*node).neighbours.insert(a);
            }
        }
    }

    pub fn nodes_in_group(&self, group: u32) -> Vec<u32> {
        let mut visited = HashSet::new();

        self.visit_nodes_from(group, &mut visited);

        visited.iter().cloned().collect()
    }

    fn visit_nodes_from(&self, index: u32, visited: &mut HashSet<u32>) {
        if visited.contains(&index) {
            return;
        }

        visited.insert(index);

        let node = self.nodes.get(&index).unwrap();

        for &n in &node.neighbours {
            self.visit_nodes_from(n, visited);
        }
    }

    pub fn groups(&self) -> Vec<u32> {
        let mut groups = Vec::new();
        let mut visited = HashSet::new();

        for &n in self.nodes.keys() {
            if visited.contains(&n) {
                continue;
            }

            groups.push(n);

            self.visit_nodes_from(n, &mut visited);
        }

        groups
    }
}

impl Node {
    fn new() -> Self {
        Node { neighbours: HashSet::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_nodes_in_group() {
        let mut graph = Graph::new();

        graph.add_node(0, &[2]);
        graph.add_node(1, &[1]);
        graph.add_node(2, &[0, 3, 4]);
        graph.add_node(3, &[2, 4]);
        graph.add_node(4, &[2, 3, 6]);
        graph.add_node(5, &[6]);
        graph.add_node(6, &[4, 5]);

        let mut group = graph.nodes_in_group(0);
        group.sort();

        assert_eq!(vec![0, 2, 3, 4, 5, 6], group);
    }

    #[test]
    fn find_groups() {
        let mut graph = Graph::new();

        graph.add_node(0, &[2]);
        graph.add_node(1, &[1]);
        graph.add_node(2, &[0, 3, 4]);
        graph.add_node(3, &[2, 4]);
        graph.add_node(4, &[2, 3, 6]);
        graph.add_node(5, &[6]);
        graph.add_node(6, &[4, 5]);

        assert_eq!(2, graph.groups().len());
    }
}
