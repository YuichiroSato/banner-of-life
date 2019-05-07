use cells::*;
use primes::*;

pub struct Compound {
    pub font_size: usize,
    pub pattern: Vec<Vec<Prime<'static>>>,
}

impl Compound {
    pub fn new(font_size: usize) -> Compound {
        Compound {
            font_size: font_size,
            pattern: vec![vec![EMPTY; font_size]; font_size],
        }
    }

    pub fn set(&mut self, prime: Prime<'static>, x: usize, y: usize) {
        self.pattern[x][y] = prime;
    }

    pub fn remove(&mut self, x: usize, y: usize) {
        self.pattern[x][y] = EMPTY;
    }

    pub fn to_cells(&self) -> Cells {
        let mut cells = Cells::new(self.font_size, self.font_size);
        for x in 0..self.font_size {
            for y in 0..self.font_size {
                let p = &self.pattern[x][y];
                if p.prime_type != PrimeType::Empty {
                    cells.allocate_prime(*p, x, y);
                }
            }
        }
        cells
    }
}
