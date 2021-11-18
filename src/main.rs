use rand;
use rand_distr;
use rand_distr::Distribution;

mod car;
mod engine;
mod graph;
mod passenger;
mod station;
mod traffic_generator;

mod routes;
mod utils;

use ndarray::Array2;

struct Poisson {
    rng: rand::prelude::ThreadRng,
    poisson: rand_distr::Poisson<f64>,
}

impl Poisson {
    fn new(lambda: f64) -> Self {
        Self {
            rng: rand::thread_rng(),
            poisson: rand_distr::Poisson::new(lambda).unwrap(),
        }
    }

    fn iter<'a>(&'a mut self, count: usize) -> impl IntoIterator<Item = u32> + 'a {
        self.poisson
            .sample_iter(&mut self.rng)
            .take(count)
            .map(|x| x as u32)
    }

    fn draw(&mut self) -> u32 {
        self.poisson.sample(&mut self.rng) as u32
    }
}

struct Assign {
    total: u32,
    poisson: Poisson,
    steps: u32,
}

impl Assign {
    fn new(total: u32, steps: u32) -> Self {
        let lambda = (total as f64) / (steps as f64);
        let lambda = if lambda > 1.0 { lambda.floor() } else { lambda };
        let poisson = Poisson::new(lambda);
        Self {
            total,
            steps,
            poisson,
        }
    }

    fn draw_next(&mut self) -> u32 {
        if self.total > 0 {
            self.help_next()
        } else {
            0
        }
    }

    fn help_next(&mut self) -> u32 {
        let n = self.poisson.draw();
        let out = (n % self.total) + 1;
        self.total -= out;
        out
    }
}

impl Iterator for Assign {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps > 0 {
            self.steps -= 1;
            Some(self.draw_next())
        } else {
            None
        }
    }
}

struct TrailingZeros {
    count: usize,
    trailing: bool,
}

impl TrailingZeros {
    fn new() -> Self {
        Self {
            count: 0,
            trailing: false,
        }
    }
    fn last(&mut self, last: u32) {
        self.trailing = last == 0;
        self.count = if self.trailing { self.count + 1 } else { 0 };
    }
}

struct RunningAverage<T>
where
    T: std::ops::Add + std::ops::AddAssign + Into<f64> + Copy,
{
    total: T,
    count: u32,
}

impl<T> RunningAverage<T>
where
    T: std::ops::Add + std::ops::AddAssign + Default + Into<f64> + Copy,
{
    fn new() -> Self {
        Self {
            total: Default::default(),
            count: 0,
        }
    }

    fn add_value(&mut self, val: T) {
        self.total += val;
        self.count += 1;
    }

    fn average(&self) -> f64 {
        let t = self.total.into();
        t / (self.count as f64)
    }
}

fn arcs_to_matrix(arcs: &[(usize, usize)]) -> Array2<u32> {
    let max = *arcs
        .iter()
        .map(|(a, b)| if a > b { a } else { b })
        .max()
        .unwrap()
        + 1;
    let mut g = Array2::from_elem((max, max), 0);
    for i in 0..max {
        for j in 0..max {
            g[(i, j)] = u32::MAX;
        }
    }

    for (i, j) in arcs.into_iter() {
        g[(*i, *j)] = 1;
        g[(*j, *i)] = 1;
    }

    for i in 0..max {
        g[(i, i)] = 0;
    }

    g
}

use std::collections::HashSet;

fn main() {
    let arcs = [
        (0, 1),
        (1, 2),
        (2, 3),
        (2, 5),
        (2, 7),
        (3, 4),
        (7, 8),
        (5, 6),
    ];
    let g = arcs_to_matrix(&arcs);

    println!("{:?}", g);
    println!("{:?}", routes::all_shortest_path::all_shortest_path(g));

    let set = HashSet::from([1, 2, 3, 4, 5, 6]);
    for _ in (0..100) {
        for i in &set {
            print!("{} ", i);
        }
        println!();
    }
}
