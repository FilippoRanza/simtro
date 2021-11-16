use rand;
use rand_distr;
use rand_distr::Distribution;

mod car;
mod engine;
mod graph;
mod passenger;
mod traffic_generator;

mod utils;

macro_rules! fdiv {
    ($a: ident, $b: ident) => {
        ($a as f64) / ($b as f64)
    };
}

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

fn main() {
    for lambda in (100..10000).step_by(100) {
        let mut poisson = Poisson::new(lambda as f64);
        let total = 100;
        let mut success = 0;
        let mut non_zero = 0;
        let mut error = RunningAverage::new();
        let mut trailing_zeros = RunningAverage::new();
        for n in poisson.iter(total) {
            let mut count = 0;

            let mut trailing = TrailingZeros::new();
            for k in Assign::new(n, 120) {
                //print!("{} ", k);
                count += k;
                trailing.last(k);
            }
            if count == n {
                success += 1;
            }
            if count > 0 {
                non_zero += 1;
            }
            trailing_zeros.add_value(trailing.count as f64);
            //println!("\n{} - {}", n, count);
            error.add_value(fdiv! {count, n});
        }
        println!("{}", lambda);
        println!("\t{}/{}", success, total);
        println!("\t{}/{}", non_zero, total);
        println!("\t{}", error.average());
        println!("\t{}", trailing_zeros.average());
        println! {};
    }
}
