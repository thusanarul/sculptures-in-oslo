use rand::{
    distr::{Distribution, Uniform},
    Rng,
};

use crate::edge::Edge;
use std::fmt::Debug;

// NOTE: Used internally by three-opt to keep track of which edges to swap
#[derive(Debug, Clone)]
enum ThreeOptCase {
    CategoryOne {
        i: usize,
        j: usize,
        delta: f32,
    },
    CategoryTwo {
        first_swap: (usize, usize),
        second_swap: (usize, usize),
        delta: f32,
    },
    CategoryThree {
        first_swap: (usize, usize),
        second_swap: (usize, usize),
        third_swap: (usize, usize),
        delta: f32,
    },
}

impl ThreeOptCase {
    fn delta(&self) -> &f32 {
        match self {
            ThreeOptCase::CategoryOne { i, j, delta } => delta,
            ThreeOptCase::CategoryTwo {
                first_swap,
                second_swap,
                delta,
            } => delta,
            ThreeOptCase::CategoryThree {
                first_swap,
                second_swap,
                third_swap,
                delta,
            } => delta,
        }
    }

    fn category_one(i: usize, j: usize, delta: f32) -> Self {
        Self::CategoryOne { i, j, delta }
    }
    fn category_two(first_swap: (usize, usize), second_swap: (usize, usize), delta: f32) -> Self {
        Self::CategoryTwo {
            first_swap,
            second_swap,
            delta,
        }
    }
    fn category_three(
        first_swap: (usize, usize),
        second_swap: (usize, usize),
        third_swap: (usize, usize),
        delta: f32,
    ) -> Self {
        Self::CategoryThree {
            first_swap,
            second_swap,
            third_swap,
            delta,
        }
    }
}

pub struct ThreeOpt<E> {
    nodes: Vec<E>,
    path: Vec<E>,
    i: usize,
    j: usize,
    k: usize,
}

impl<E> ThreeOpt<E>
where
    E: Edge + Clone + Debug,
{
    pub fn new(nodes: Vec<E>, path: Vec<E>) -> Self {
        Self {
            nodes,
            path,
            i: 0,
            j: 0,
            k: 0,
        }
    }

    pub fn path(&self) -> &Vec<E> {
        &self.path
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

    fn dist(&self, index_1: usize, index_2: usize) -> f32 {
        let path = &self.path;
        path[index_1].weight(&path[index_2])
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

    fn swap_edges_by_case(&mut self, case: &ThreeOptCase) {
        match case {
            ThreeOptCase::CategoryOne { i, j, delta } => {
                self.swap_edges(*i, *j);
            }
            ThreeOptCase::CategoryTwo {
                first_swap,
                second_swap,
                delta,
            } => {
                self.swap_edges(first_swap.0, first_swap.1);
                self.swap_edges(second_swap.0, second_swap.1);
            }
            ThreeOptCase::CategoryThree {
                first_swap,
                second_swap,
                third_swap,
                delta,
            } => {
                self.swap_edges(first_swap.0, first_swap.1);
                self.swap_edges(second_swap.0, second_swap.1);
                self.swap_edges(third_swap.0, third_swap.1);
            }
        }
    }

    fn get_best_case(
        &self,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    ) -> ThreeOptCase {
        // Gain: Length of added edges - length of removed edges

        // Two-opt cases
        let delta_case_1 = ThreeOptCase::category_one(
            c,
            e,
            -self.dist(c, d) - self.dist(e, f) + self.dist(c, e) + self.dist(d, f),
        );

        // fixed: c-d
        let delta_case_2 = ThreeOptCase::category_one(
            a,
            e,
            -self.dist(a, b) - self.dist(e, f) + self.dist(a, e) + self.dist(b, f),
        );

        // fixed: e-f
        let delta_case_3 = ThreeOptCase::category_one(
            a,
            c,
            -self.dist(a, b) - self.dist(c, d) + self.dist(a, c) + self.dist(b, d),
        );

        // Three-opt cases

        let common = -self.dist(a, b) - self.dist(c, d) - self.dist(e, f);

        let delta_case_4 = ThreeOptCase::category_two(
            (a, d),
            (b, e),
            common + self.dist(a, d) + self.dist(e, b) + self.dist(f, c),
        );

        let delta_case_5 = ThreeOptCase::category_two(
            (a, e),
            (c, e),
            common + self.dist(a, e) + self.dist(d, b) + self.dist(c, f),
        );

        let delta_case_6 = ThreeOptCase::category_two(
            (a, c),
            (c, e),
            common + self.dist(a, c) + self.dist(b, e) + self.dist(d, f),
        );

        let delta_case_7 = ThreeOptCase::category_three(
            (a, d),
            (b, e),
            (c, e),
            common + self.dist(a, d) + self.dist(e, c) + self.dist(b, f),
        );

        let cases = vec![
            delta_case_1,
            delta_case_2,
            delta_case_3,
            delta_case_4,
            delta_case_5,
            delta_case_6,
            delta_case_7,
        ];

        let mut current_best = &cases[0];
        for case in cases[1..].iter() {
            // NOTE: swaps the case if we have found a better case
            if case.delta() < current_best.delta() {
                current_best = case
            }
        }

        current_best.clone()
    }

    // Choose any three edges and reconnect them in all the ways there are to combine them that do not produce cycles.
    pub fn solve(&mut self) -> Vec<E> {
        let n = self.path().len();

        let mut cost = self.calculate_path_cost();

        let mut found_improvement = true;
        while found_improvement {
            found_improvement = false;

            'outer: for i in 0..(n - 1) {
                let a = i;
                let b = (i + 1) % n;
                for j in (i + 2)..n {
                    let c = j;
                    let d = (j + 1) % n;

                    for k in (j + 2)..n {
                        let e = k;
                        let f = (k + 1) % n;

                        let best_case = self.get_best_case(a, b, c, d, e, f);

                        // Only accept the best case if it improves the path cost
                        if best_case.delta() < &-0.001 {
                            found_improvement = true;

                            self.swap_edges_by_case(&best_case);
                            cost += best_case.delta();

                            break 'outer;
                        }
                    }
                }
            }
        }

        self.path.clone()
    }

    // Three-opt based simulated annealing
    // https://en.wikipedia.org/wiki/Simulated_annealing
    // https://optimization.cbe.cornell.edu/index.php?title=Simulated_annealing
    // file:///Users/thusanarul/Documents/three-opt-simulated-annealing.pdf
    // https://algorithmafternoon.com/books/simulated_annealing/chapter01/
    pub fn solve_sm(&mut self) -> Vec<E> {
        // Cool down rate constant
        let alpha = 0.9;

        let n = self.path().len();

        // Random sampling
        let between = Uniform::<f32>::try_from(0.0..=1.0).unwrap();
        let mut rng = rand::rng();

        let mut cost = self.calculate_path_cost();

        let mut temperature: f32 = 100000.0;
        while temperature.trunc() > 0.0 {
            'outer: for i in 0..(n - 1) {
                let a = i;
                let b = (i + 1) % n;
                for j in (i + 2)..n {
                    let c = j;
                    let d = (j + 1) % n;

                    for k in (j + 2)..n {
                        let e = k;
                        let f = (k + 1) % n;

                        let best_case = self.get_best_case(a, b, c, d, e, f);

                        println!("Best case: {:#?}", best_case);

                        let delta = best_case.delta();
                        // NOTE: For simulated annealing
                        let current_temperature = temperature;
                        temperature = alpha * current_temperature;

                        let rand = between.sample(&mut rng);
                        let prob = (-delta / current_temperature).exp();

                        // We will then select the best case if:
                        // - better than the current solution
                        // - - Which in our case will be true if delta is negative
                        // - or if it passes a certain probability according to the SA approach

                        if delta < &-0.001 {
                            println!("Chose improving solution: {}", delta);
                            self.swap_edges_by_case(&best_case);
                            cost += best_case.delta();
                            break 'outer;
                        } else if rand <= prob {
                            println!(
                                "Chose worse solution: {} {} {} {}",
                                delta, temperature, rand, prob
                            );
                            self.swap_edges_by_case(&best_case);
                            cost += best_case.delta();
                            break 'outer;
                        }
                    }
                }
            }
        }

        self.path.clone()
    }
}
