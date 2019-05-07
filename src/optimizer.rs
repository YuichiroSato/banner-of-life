use mold::*;
use cells::*;
use compound::*;
use primes::*;
use randomizer::*;

pub trait Optimizer {
    fn optimize(&mut self, mold: Mold) -> Compound;
}

pub struct GradientDescent<T: Randomizer> {
    pub randomizer: T,
    pub n: usize,
}

impl<T: Randomizer> Optimizer for GradientDescent<T> {
    fn optimize(&mut self, mold: Mold) -> Compound {
        let mut pattern = Compound::new(mold.font_size);
        let mut occupied = Cells::new(mold.font_size, mold.font_size);
        let mut score;

        self.add_primes(&mold, &mut pattern, &mut occupied);
        self.allocate_compound(&mut occupied, &pattern);
        score = self.evaluate(&mold, &pattern);
        for _ in 0..self.n {
            let mut tmp = Compound {
                font_size: pattern.font_size,
                pattern: pattern.pattern.clone(),
            };
            self.remove_primes(&mold, &mut tmp);
            occupied.clear();
            self.allocate_compound(&mut occupied, &tmp);
            self.add_primes(&mold, &mut tmp, &mut occupied);
            let mut tmp_score = self.evaluate(&mold, &tmp);
            if score < tmp_score {
                pattern = tmp;
                score = tmp_score;
            }
        }
        pattern
    }
}

impl<T: Randomizer> GradientDescent<T> {
    fn remove_primes(&mut self, mold: &Mold, pattern: &mut Compound) {
        for x in 0..mold.font_size {
            for y in 0..mold.font_size {
                if pattern.pattern[x][y].prime_type != PrimeType::Empty {
                    if self.randomizer.random_number() < 0.1 {
                        pattern.remove(x, y);
                    }
                }
            }
        }
    }

    fn add_primes(&mut self, mold: &Mold, pattern: &mut Compound, occupied: &mut Cells) {
        for x in 0..mold.font_size {
            for y in 0..mold.font_size {
                if !mold.target.is_allocatable(x, y, 4) && self.randomizer.random_number() < 0.05 {
                    let p = self.get_random_prime();
                    if occupied.is_allocatable(x, y, p.exclusive_size) {
                        pattern.set(p, x, y);
                        occupied.make_alive_square(x, y, p.exclusive_size);
                    }
                }
            }
        }
    }

    fn allocate_compound(&self, occupied: &mut Cells, compound: &Compound) {
        for x in 0..compound.font_size {
            for y in 0..compound.font_size {
                if compound.pattern[x][y].prime_type != PrimeType::Empty {
                    occupied.make_alive_square(x, y, compound.pattern[x][y].exclusive_size);
                }
            }
        }
    }

    fn get_random_prime(&mut self) -> Prime<'static> {
        let n = (self.randomizer.random_number() * 7.0) as usize;
        match n {
            0 => BLOCK,
            1 => TUB,
            2 => BEEHIVE,
            3 => BLINKER,
            4 => BEACON,
            5 => CLOCK,
            _ => TOAD,
        }
    }

    fn evaluate(&self, mold: &Mold, pattern: &Compound) -> f64 {
        let cells = pattern.to_cells();
        let a = mold.intersect(&cells);
        let b = mold.difference(&cells);
        a as f64 - (b as f64) * 0.5
    }
}
