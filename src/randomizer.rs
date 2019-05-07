use rand::prelude::*;

pub trait Randomizer {
    fn new() -> Self;
    fn random_number(&mut self) -> f64;
}

pub struct Rand {
    source: ThreadRng
}

impl Randomizer for Rand {
    fn new() -> Self {
        Rand {
            source: rand::thread_rng()
        }
    }

    fn random_number(&mut self) -> f64 {
        self.source.gen()
    }
}
