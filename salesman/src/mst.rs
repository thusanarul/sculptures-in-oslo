// TODO
// consider creating 1-tree which gives a better lower bound to compare heuristics.

use std::collections::VecDeque;
use std::fmt::Debug;

use crate::edge::Edge;

#[derive(Debug, Clone)]
struct TreeEdge<N: Edge> {
    a: N,
    b: N,
}

impl<N: Edge> TreeEdge<N> {
    fn cost(&self) -> f32 {
        self.a.weight(&self.b)
    }
}

#[derive(Debug)]
pub struct MST<E: Edge + Clone> {
    nodes: Vec<E>,
    edges: Vec<TreeEdge<E>>,
}

impl<E: Edge + Clone + Debug> MST<E> {
    pub fn new(nodes: Vec<E>) -> Self {
        Self {
            nodes,
            edges: vec![],
        }
    }

    // Finds MST by using Prim's algo
    // Assumes first node is the start node. could change this to an arbitrary node
    pub fn solve(&mut self) {
        let mut nodes = VecDeque::from(self.nodes.clone());
        let input_nodes_len = nodes.len();

        let mut visited: Vec<E> = vec![];
        let mut edges: Vec<TreeEdge<E>> = vec![];

        if let Some(node) = nodes.pop_front() {
            visited.push(node);
        }

        while visited.len() != input_nodes_len {
            let mut best_dist = f32::INFINITY;
            let mut closest_node_index: usize = 0;
            let mut closest_edge: Option<TreeEdge<E>> = None;

            for i in 0..visited.len() {
                let v = visited[i].clone();
                for j in 0..nodes.len() {
                    let u = &nodes[j];
                    if v.weight(u) < best_dist {
                        best_dist = v.weight(u);
                        closest_node_index = j;

                        closest_edge = Some(TreeEdge {
                            a: v.clone(),
                            b: u.clone(),
                        });
                    }
                }
            }

            if let Some(found) = closest_edge.clone() {
                edges.push(found.clone());

                // NOTE: b is the new node in the edge
                visited.push(found.b.clone());
                nodes.remove(closest_node_index);
            }
        }

        self.edges = edges;
    }

    pub fn calculate_cost(&self) -> f32 {
        self.edges.iter().fold(0.0, |acc, curr| acc + curr.cost())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Node {
        weight: i32,
    }

    impl Node {
        fn new(weight: i32) -> Self {
            Self { weight }
        }
    }

    impl Edge for Node {
        fn weight(&self, node: &Self) -> f32 {
            (self.weight + node.weight) as f32
        }
    }

    fn simple_nodes() -> Vec<Node> {
        vec![
            Node::new(5),
            Node::new(4),
            Node::new(1),
            Node::new(3),
            Node::new(2),
            Node::new(6),
        ]
    }

    #[test]
    fn prims() {
        let mut mst = MST::new(simple_nodes());
        mst.solve();
        let cost = mst.calculate_cost();
        assert_eq!(cost, 25.0);
    }
}
