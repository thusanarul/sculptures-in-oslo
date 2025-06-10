use std::fmt::Debug;

use crate::{edge::Edge, tsp_ext::three_opt::ThreeOpt};

pub struct TSP<E: Edge + Clone> {
    nodes: Vec<E>,
    path: Vec<E>,
}

impl<E: Edge + Clone + Debug> TSP<E> {
    pub fn new(nodes: Vec<E>) -> Self {
        Self {
            nodes,
            path: vec![],
        }
    }

    pub fn new_and_initialize_path(nodes: Vec<E>) -> Self {
        Self {
            nodes: nodes.clone(),
            path: nodes,
        }
    }

    pub fn path(&self) -> &Vec<E> {
        &self.path
    }

    fn distance_matrix(&self) -> Vec<Vec<f32>> {
        let n = self.nodes.len();

        let mut m: Vec<Vec<f32>> = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                m[i][j] = self.nodes[i].weight(&self.nodes[j]);
            }
        }

        m
    }

    // Replaces edges path[i]->path[i+1] and path[j]->path[j+1]
    // with path[i]->path[j] and path[i+1]->path[j+1]
    fn swap_edges(&mut self, i: usize, j: usize) {
        let mut i = i;
        let mut j = j;

        i += 1;

        while i < j {
            let tmp = self.path[i].clone();
            self.path[i] = self.path[j].clone();
            self.path[j] = tmp;

            i += 1;
            j -= 1;
        }
    }

    fn dist(&self, index_1: usize, index_2: usize) -> f32 {
        let path = &self.path;
        path[index_1].weight(&path[index_2])
    }

    // NOTE: Needs a path to optimize
    pub fn two_opt(&mut self) -> Vec<E> {
        let n = self.path().len();

        // Total cost of path
        let mut cost = self.calculate_path_cost();
        let mut found_improvement = true;

        while found_improvement {
            found_improvement = false;

            for i in 0..(n - 1) {
                let a = i;
                let b = (i + 1) % n;
                for j in (i + 2)..n {
                    let c = j;
                    let d = (j + 1) % n;
                    // Calculate delta change if connections are switched
                    let length_delta =
                        -self.dist(a, b) - self.dist(c, d) + self.dist(a, c) + self.dist(b, d);

                    if length_delta < -0.001 {
                        self.swap_edges(a, c);
                        cost += length_delta;
                        found_improvement = true;
                    }
                }
            }
        }

        self.path().clone()
    }

    pub fn three_opt(&mut self) -> Vec<E> {
        self.path = ThreeOpt::new(self.nodes.clone(), self.path.clone()).solve();
        self.path.clone()
    }

    pub fn three_opt_sm(&mut self) -> Vec<E> {
        self.path = ThreeOpt::new(self.nodes.clone(), self.path.clone()).solve_sm();
        self.path.clone()
    }

    // Nearest neighbour
    // Currently assumes first node is the starting point, but could start at random point.
    pub fn nn(&mut self) -> Vec<E> {
        let mut path: Vec<E> = vec![];

        let mut nodes_to_visit = self.nodes.clone();

        let initial = nodes_to_visit.remove(0);
        let mut current_node = initial.clone();
        path.push(initial);

        while !nodes_to_visit.is_empty() {
            let mut current_shortest = f32::MAX;
            let mut current_index: usize = usize::MIN;

            for (i, node) in nodes_to_visit.iter().enumerate() {
                let weight = current_node.weight(node);

                if weight < current_shortest {
                    current_shortest = weight;
                    current_index = i;
                }
            }

            let chosen = nodes_to_visit.remove(current_index);
            current_node = chosen.clone();
            path.push(chosen);
        }

        path.push(path[0].clone());

        self.path = path.clone();

        path
    }

    pub fn calculate_path_cost(&self) -> f32 {
        let path = &self.path;
        let n = path.len();
        let mut cost = self.dist(path.len() - 1, 0);

        for i in 0..n - 1 {
            let dist = self.dist(i, i + 1);
            cost += dist;
        }

        cost
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[derive(Clone, Debug, PartialEq)]
    struct OneDPoint {
        x: f32,
    }

    impl OneDPoint {
        fn new(x: f32) -> Self {
            Self { x }
        }
    }

    impl Edge for OneDPoint {
        fn weight(&self, node: &Self) -> f32 {
            let diffx = self.x - node.x;

            (diffx * diffx).sqrt()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Point {
        x: f32,
        y: f32,

        // This number indicates where in the path this node should appear in calculated path
        index: i32,
    }

    impl Point {
        pub fn new(x: f32, y: f32, index: i32) -> Self {
            Self { x, y, index }
        }

        // Distance between two points
        fn distance_to(&self, other: &Point) -> f32 {
            let diffx = self.x - other.x;
            let diffy = self.y - other.y;

            ((diffx * diffx) + (diffy * diffy)).sqrt()
        }
    }

    impl Edge for Point {
        fn weight(&self, point: &Self) -> f32 {
            self.distance_to(point)
        }
    }

    struct WikiPaths {
        path_1: Vec<Point>,
        path_2: Vec<Point>,
    }

    fn wiki_nodes() -> eyre::Result<WikiPaths> {
        let dir = format!("{}/src/tests", env!("CARGO_MANIFEST_DIR"));
        // source: https://en.wikipedia.org/wiki/2-opt
        let path_1: Vec<Point> = fs::read_to_string(format!("{}/path_1", dir))?
            .split("\n")
            .enumerate()
            .filter_map(|(index, line)| {
                let mut line = line.splitn(2, ",");

                let x = line.next().and_then(|v| v.parse::<f32>().ok())?;
                let y = line.next().and_then(|v| v.parse::<f32>().ok())?;

                Some(Point::new(x, y, index as i32))
            })
            .collect();

        // source: https://en.wikipedia.org/wiki/2-opt
        let path_2 = fs::read_to_string(format!("{}/path_2", dir))?
            .split("\n")
            .enumerate()
            .filter_map(|(index, line)| {
                let mut line = line.splitn(2, ",");

                let x = line.next().and_then(|v| v.parse::<f32>().ok())?;
                let y = line.next().and_then(|v| v.parse::<f32>().ok())?;

                Some(Point::new(x, y, index as i32))
            })
            .collect();

        Ok(WikiPaths { path_1, path_2 })
    }

    #[test]
    fn path_cost() {
        let node_1 = Point::new(0.0, 0.0, 0);
        let node_2 = Point::new(1.0, 1.0, 1);
        let node_3 = Point::new(2.0, 2.0, 2);
        let node_4 = Point::new(3.0, 3.0, 3);
        let node_5 = Point::new(4.0, 4.0, 4);
        let node_6 = Point::new(5.0, 5.0, 5);

        let nodes = vec![node_1, node_2, node_3, node_4, node_5, node_6];
        let tsp = TSP::new_and_initialize_path(nodes);
        let cost = tsp.calculate_path_cost();

        assert_eq!(cost, 14.142134)
    }

    #[test]
    fn nn() {
        let node_1 = Point::new(0.0, 1.0, 0);
        let node_2 = Point::new(1.0, 5.0, 3);
        let node_3 = Point::new(3.0, 4.0, 2);
        let node_4 = Point::new(8.0, 2.0, 5);
        let node_5 = Point::new(3.0, 3.0, 1);
        let node_6 = Point::new(6.0, 5.0, 4);

        let nodes = vec![node_1, node_2, node_3, node_4, node_5, node_6];

        let mut tsp = TSP::new(nodes);
        let path = tsp.nn();

        let indexed_path: Vec<i32> = path.iter().map(|node| node.index).collect();

        assert_eq!(vec![0, 1, 2, 3, 4, 5, 0], indexed_path);

        let cost = tsp.calculate_path_cost();

        assert_eq!(cost, 22.0);
    }

    #[test]
    fn two_opt_wiki() -> eyre::Result<()> {
        let WikiPaths { path_1, path_2 } = wiki_nodes()?;
        let mut tsp = TSP::new_and_initialize_path(path_2);

        let cost = tsp.calculate_path_cost();
        assert_eq!(cost.floor(), 55723.0);

        let _ = tsp.two_opt();

        let cost = tsp.calculate_path_cost();

        assert_eq!(cost.floor(), 8559.0);

        let mut tsp2 = TSP::new_and_initialize_path(path_1);
        let cost = tsp2.calculate_path_cost();
        assert_eq!(cost.floor(), 8586.0);

        let _ = tsp2.two_opt();

        let cost = tsp.calculate_path_cost();

        assert_eq!(cost.floor(), 8559.0);

        Ok(())
    }

    #[test]
    fn three_opt_wiki() -> eyre::Result<()> {
        let WikiPaths { path_1, path_2 } = wiki_nodes()?;
        let mut tsp = TSP::new(path_2);
        tsp.nn();

        let prev_cost = tsp.calculate_path_cost();
        // assert_eq!(prev_cost.floor(), 55723.0);

        let _ = tsp.three_opt();

        let cost = tsp.calculate_path_cost();

        assert!(cost < prev_cost);

        assert!(cost.floor() < 8559.0);

        println!("Cost: {cost}");

        let mut tsp2 = TSP::new(path_1);
        tsp2.nn();

        let prev_cost = tsp2.calculate_path_cost();

        let _ = tsp2.three_opt();

        let cost = tsp2.calculate_path_cost();
        assert!(cost < prev_cost);

        println!("Cost: {cost}");
        assert!(cost.floor() < 8559.0);
        Ok(())
    }

    #[test]
    fn three_opt_sm_wiki() -> eyre::Result<()> {
        let WikiPaths { path_1, path_2 } = wiki_nodes()?;
        let mut tsp = TSP::new(path_2);
        tsp.nn();

        let prev_cost = tsp.calculate_path_cost();
        // assert_eq!(prev_cost.floor(), 55723.0);

        let _ = tsp.three_opt_sm();

        let cost = tsp.calculate_path_cost();

        println!("Cost: {cost}");

        assert!(cost < prev_cost);
        assert!(cost.floor() < 8559.0);

        let mut tsp2 = TSP::new(path_1);
        tsp2.nn();

        let prev_cost = tsp2.calculate_path_cost();

        let _ = tsp2.three_opt_sm();

        let cost = tsp2.calculate_path_cost();
        println!("Cost: {cost}");

        assert!(cost < prev_cost);
        assert!(cost.floor() < 8559.0);
        Ok(())
    }

    #[test]
    fn swap_edges() -> eyre::Result<()> {
        let path = vec![
            OneDPoint::new(1.0),
            OneDPoint::new(2.0),
            OneDPoint::new(3.0),
            OneDPoint::new(4.0),
            OneDPoint::new(5.0),
            OneDPoint::new(6.0),
            OneDPoint::new(7.0),
            OneDPoint::new(8.0),
        ];

        let mut tsp = TSP::new_and_initialize_path(path);
        tsp.swap_edges(0, 3);
        println!("path: {:#?}", tsp.path());
        tsp.swap_edges(1, 4);
        println!("path: {:#?}", tsp.path());
        tsp.swap_edges(2, 4);

        println!("path: {:#?}", tsp.path());
        // tsp.swap_edges(1, 4);

        // assert_eq!(
        //     &vec![
        //         OneDPoint::new(1.0),
        //         OneDPoint::new(3.0),
        //         OneDPoint::new(2.0),
        //         OneDPoint::new(4.0),
        //         OneDPoint::new(5.0),
        //         OneDPoint::new(6.0),
        //         OneDPoint::new(7.0),
        //         OneDPoint::new(8.0),
        //     ],
        //     tsp.path()
        // );

        Ok(())
    }
}
