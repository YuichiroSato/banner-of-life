use fonts::*;
use primes::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cells {
    pub size_x: usize,
    pub size_y: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Cells {
    pub fn new(size_x: usize, size_y: usize) -> Cells {
        Cells {
            size_x: size_x,
            size_y: size_y,
            cells: vec![vec![Cell::Dead; size_y]; size_x],
        }
    }

    pub fn from_vec(cells: Vec<Vec<u8>>) -> Self {
        let size_x = cells.len();
        let size_y = cells.iter()
            .map(|v| v.len())
            .max()
            .unwrap();
        let mut normalized = vec![vec![Cell::Dead; size_y]; size_x];

        for x in 0..cells.len() {
            for y in 0..cells[x].len() {
                if cells[x][y] > 0 {
                    normalized[y][x] = Cell::Alive;
                }
            }
        }

        Cells {
            size_x: size_x,
            size_y: size_y,
            cells: normalized,
        }
    }

    pub fn from_font(font: Font) -> Self {
        let size_x = font.val.len();
        let size_y = font.val[0].len();
        let mut normalized = vec![vec![Cell::Dead; size_y]; size_x];

        for x in 0..size_x {
            for y in 0..size_y {
                if font.val[x][y] > 0 {
                    normalized[y][x] = Cell::Alive;
                }
            }
        }

        Cells {
            size_x: size_x,
            size_y: size_y,
            cells: normalized,
        }
    }

    pub fn cell_x(&self, x: i64) -> usize {
        let sx = self.size_x as i64;
        (((x % sx) + sx) % sx) as usize
    }

    pub fn cell_y(&self, y: i64) -> usize {
        let sy = self.size_y as i64;
        (((y % sy) + sy) % sy) as usize
    }

    pub fn make_alive(&mut self, x: usize, y: usize) {
        let cx = self.cell_x(x as i64);
        let cy = self.cell_y(y as i64);
        self.cells[cx][cy] = Cell::Alive;
    }

    pub fn make_alive_square(&mut self, x: usize, y: usize, size: usize) {
        for _x in x..(x + size) {
            for _y in y..(y + size) {
                self.make_alive(_x, _y);
            }
        }
    }

    pub fn make_dead(&mut self, x: usize, y: usize) {
        let cx = self.cell_x(x as i64);
        let cy = self.cell_y(y as i64);
        self.cells[cx][cy] = Cell::Dead;
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        let cx = self.cell_x(x as i64);
        let cy = self.cell_y(y as i64);
        self.cells[cx][cy] == Cell::Alive
    }

    pub fn clear(&mut self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                self.make_dead(x, y);
            }
        }
    }

    pub fn is_allocatable(&self, x: usize, y: usize, size: usize) -> bool {
        let mut able = true;

        for _x in x..(x + size) {
            for _y in y..(y + size) {
                able &= !self.is_alive(_x, _y);
            }
        }

        able
    }

    pub fn allocate(&mut self, cells: Cells, x: usize, y: usize, scale: f64) {
        let scaled_x = cells.size_x * scale as usize;
        let scaled_y = cells.size_y * scale as usize;
        for x2 in 0..scaled_x {
            for y2 in 0..scaled_y {
                let fx = (x2 as f64 / scale) as usize;
                let fy = (y2 as f64 / scale) as usize;
                if cells.is_alive(fx, fy) {
                    self.make_alive(x + x2, y + y2);
                } else {
                    self.make_dead(x + x2, y + y2);
                }
            }
        }
    }

    pub fn allocate_prime(&mut self, prime: Prime, x: usize, y: usize) {
        for x2 in 0..prime.exclusive_size {
            for y2 in 0..prime.exclusive_size {
                if prime.prime[x2][y2] > 0 {
                    self.make_alive(x + x2, y + y2);
                } else {
                    self.make_dead(x + x2, y + y2);
                }
            }
        }
    }
}

#[test]
fn test_new() {
    let cells = Cells::new(2, 2);

    assert_eq!(2, cells.size_x);
    assert_eq!(2, cells.size_y);
    assert_eq!(vec![vec![Cell::Dead; 2]; 2], cells.cells);
}

#[test]
fn test_from_vec() {
    let cells = Cells::from_vec(
        vec![
            vec![1, 1, 1],
            vec![1, 0, 0],
            vec![0, 0, 0]
        ]);

    assert_eq!(3, cells.size_x);
    assert_eq!(3, cells.size_y);
    assert_eq!(
        vec![
            vec![Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Dead, Cell::Dead]
        ],
         cells.cells);
}

#[test]
fn test_cell_x() {
    let cells = Cells::new(10, 5);

    assert_eq!(1, cells.cell_x(1));
    assert_eq!(2, cells.cell_x(12));
    assert_eq!(9, cells.cell_x(-1));
    assert_eq!(8, cells.cell_x(-12));
}

#[test]
fn test_cell_y() {
    let cells = Cells::new(10, 5);

    assert_eq!(1, cells.cell_y(1));
    assert_eq!(2, cells.cell_y(7));
    assert_eq!(4, cells.cell_y(-1));
    assert_eq!(3, cells.cell_y(-7));
}

#[test]
fn test_make_alive() {
    let mut cells = Cells::new(2, 2);
    cells.make_alive(0, 0);
    cells.make_alive(0, 1);

    let expected = Cells::from_vec(vec![vec![1, 0], vec![1, 0]]);

    assert_eq!(expected, cells);
}

#[test]
fn test_make_alive_square() {
    let mut cells = Cells::new(5, 5);
    cells.make_alive_square(2, 3, 2);

    let expected = Cells::from_vec(vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 0],
    ]);

    assert_eq!(expected, cells);
}

#[test]
fn test_make_dead() {
    let mut cells = Cells::from_vec(vec![vec![1, 1], vec![1, 1]]);
    cells.make_dead(1, 0);
    cells.make_dead(1, 1);

    let expected = Cells::from_vec(vec![vec![1, 0], vec![1, 0]]);

    assert_eq!(expected, cells);
}

#[test]
fn test_is_alive() {
    let cells = Cells::from_vec(vec![vec![1, 0], vec![1, 0]]);

    assert!(cells.is_alive(0, 0));
    assert!(!cells.is_alive(1, 0));
    assert!(cells.is_alive(0, 1));
    assert!(!cells.is_alive(1, 1));
}

#[test]
fn test_clear() {
    let mut cells = Cells::from_vec(vec![vec![1, 1], vec![1, 1]]);
    cells.clear();

    let expected = Cells::new(2, 2);

    assert_eq!(expected, cells);
}

#[test]
fn test_is_allocatable() {
    let cells = Cells::from_vec(vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);

    assert!(cells.is_allocatable(1, 2, 2));
    assert!(!cells.is_allocatable(3, 3, 2));
}

#[test]
fn test_allocate() {
    let mut cells = Cells::new(10, 10);
    let font = Cells::from_vec(vec![vec![1, 1], vec![1, 0]]);

    cells.allocate(font, 4, 4, 2.0);
    let expected = Cells::from_vec(vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);

    assert_eq!(expected, cells);
}

#[test]
fn test_allocate_prime() {
    let mut cells = Cells::new(10, 10);

    cells.allocate_prime(BEACON, 2, 2);
    let expected = Cells::from_vec(vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);

    assert_eq!(expected, cells);
}
