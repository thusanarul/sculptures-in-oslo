use std::{fmt::Debug, i32};

pub trait Edge {
    fn weight(&self, node: &Self) -> i32;
}

struct TSP<E: Edge + Clone> {
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

    fn path(&self) -> Vec<E> {
        self.path.clone()
    }

    fn distance_matrix(&self) -> Vec<Vec<i32>> {
        let n = self.nodes.len();

        let mut m: Vec<Vec<i32>> = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                m[i][j] = self.nodes[i].weight(&self.nodes[j]);
            }
        }

        return m;
    }

    // Replaces edges path[i]->path[i+1] and path[j]->path[j+1]
    // with path[i]->path[j] and path[i+1]->path[j+1]
    fn swap_edges(&mut self, mut i: usize, mut j: usize) {
        i = i + 1;

        while i < j {
            let tmp = self.path[i].clone();
            self.path[i] = self.path[j].clone();
            self.path[j] = tmp;

            i = i + 1;
            j = j - 1;
        }
    }

    fn dist(&self, index_1: usize, index_2: usize) -> i32 {
        let path = &self.path;
        path[index_1].weight(&path[index_2])
    }

    pub fn two_opt(&mut self) -> Vec<E> {
        let n = self.path().len();

        let mut cost = self.calculate_path_cost();

        let mut found_improvement = true;

        while found_improvement {
            found_improvement = false;

            for i in 0..(n - 1) {
                for j in (i + 2)..n {
                    // Calculate delta change if connections are switched
                    let length_delta = -self.dist(i, (i + 1) % n) - self.dist(j, (j + 1) % n)
                        + self.dist(i, j)
                        + self.dist(i + 1, (j + 1) % n);

                    if length_delta < 0 {
                        self.swap_edges(i, j);
                        cost = cost + length_delta;
                        found_improvement = true;
                    }
                }
            }
        }

        self.path().clone()
    }

    // Nearest neighbour
    // Currently assumes first node is the starting point, but could start at random point.
    pub fn nn(&mut self) -> Vec<E> {
        let mut path: Vec<E> = vec![];

        let mut nodes_to_visit = self.nodes.clone();

        let initial = nodes_to_visit.remove(0);
        let mut current_node = initial.clone();
        path.push(initial);

        while nodes_to_visit.len() != 0 {
            let mut current_shortest = i32::MAX;
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

        return path;
    }

    pub fn calculate_path_cost(&self) -> i32 {
        let path = &self.path;
        let n = path.len();
        let mut cost = self.dist(path.len() - 1, 0);

        for i in 0..(n - 1) {
            cost = cost + self.dist(i, i + 1)
        }

        return cost;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,

        // This number indicates where in the path this node should appear in calculated path
        index: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32, index: i32) -> Self {
            Self { x, y, index }
        }

        fn distance_to(&self, target: &Point) -> i32 {
            let diffx = self.x - target.x;
            let diffy = self.y - target.y;

            (((diffx * diffx) + (diffy * diffy)) as f32).sqrt().floor() as i32
        }
    }

    impl Edge for Point {
        fn weight(&self, point: &Self) -> i32 {
            self.distance_to(point)
        }
    }

    #[test]
    fn test() {
        let node_1 = Point::new(0, 1, 0);
        let node_2 = Point::new(1, 5, 3);
        let node_3 = Point::new(3, 4, 2);
        let node_4 = Point::new(8, 2, 5);
        let node_5 = Point::new(3, 3, 1);
        let node_6 = Point::new(6, 5, 4);

        let nodes = vec![node_1, node_2, node_3, node_4, node_5, node_6];

        let mut tsp = TSP::new(nodes);
        let path = tsp.nn();

        let indexed_path: Vec<i32> = path.iter().map(|node| node.index).collect();

        assert_eq!(vec![0, 1, 2, 3, 4, 5, 0], indexed_path);

        let cost = tsp.calculate_path_cost();

        assert_eq!(cost, 22);

        let path = tsp.two_opt();

        let indexed_path: Vec<i32> = path.iter().map(|node| node.index).collect();

        dbg!(&indexed_path);
        // assert_eq!(vec![0, 2, 1, 3, 4, 5, 0], indexed_path);

        let cost = tsp.calculate_path_cost();
        assert_eq!(cost, 20);
    }
}
