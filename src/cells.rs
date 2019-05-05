#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub trait Cells {
    fn new(size_x: usize, size_y: usize) -> Self;
    fn replicate(&self) -> Self;
    fn make_alive(&mut self, x: usize, y: usize);
    fn make_dead(&mut self, x: usize, y: usize);
    fn is_alive(&self, x: usize, y: usize) -> bool;
    fn count_alive_around(&self, x: usize, y: usize) -> i32;
}

pub struct CellsImpl {
    pub size_x: usize,
    pub size_y: usize,
    cells: Vec<Vec<i32>>,
}

impl CellsImpl {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        CellsImpl {
            size_x: size_x,
            size_y: size_y,
            cells: vec![vec![0; size_y]; size_x],
        }
    }

    pub fn replicate(&self) -> Self {
        CellsImpl {
            size_x: self.size_x,
            size_y: self.size_y,
            cells: vec![vec![0; self.size_y]; self.size_x],
        }
    }

    pub fn make_alive(&mut self, x: usize, y: usize) {
        let cx = self.cell_x(x as i32);
        let cy = self.cell_y(y as i32);
        self.cells[cx][cy] = 1;
    }

    pub fn make_dead(&mut self, x: usize, y: usize) {
        let cx = self.cell_x(x as i32);
        let cy = self.cell_y(y as i32);
        self.cells[cx][cy] = 0;
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        let cx = self.cell_x(x as i32);
        let cy = self.cell_y(y as i32);
        self.cells[cx][cy] == 1
    }

    pub fn count_alive_around(&self, x: usize, y: usize) -> i32 {
        let left = self.cell_x(x as i32 - 1);
        let right = self.cell_x(x as i32 + 1);
        let upper = self.cell_y(y as i32 - 1);
        let lower = self.cell_y(y as i32 + 1);

        self.cells[left][upper]
            + self.cells[x][upper]
            + self.cells[right][upper]
            + self.cells[left][y]
            + self.cells[right][y]
            + self.cells[left][lower]
            + self.cells[x][lower]
            + self.cells[right][lower]
    }
}

impl CellsImpl {
    fn cell_x(&self, x: i32) -> usize {
        let sx = self.size_x as i32;
        (((x % sx) + sx) % sx) as usize
    }

    fn cell_y(&self, y: i32) -> usize {
        let sy = self.size_y as i32;
        (((y % sy) + sy) % sy) as usize
    }
}

#[test]
fn test_count_alive_around() {
    let mut cells = CellsImpl::new(10, 5);
    cells.make_alive(0, 1);
    cells.make_alive(0, 2);
    cells.make_alive(1, 0);
    cells.make_alive(2, 2);
    cells.make_alive(9, 0);
    cells.make_alive(2, 4);

    assert_eq!(3, cells.count_alive_around(0, 0));
    assert_eq!(2, cells.count_alive_around(1, 0));
    assert_eq!(2, cells.count_alive_around(2, 0));
    assert_eq!(1, cells.count_alive_around(3, 0));
    assert_eq!(3, cells.count_alive_around(0, 1));
    assert_eq!(4, cells.count_alive_around(1, 1));
    assert_eq!(2, cells.count_alive_around(2, 1));
    assert_eq!(1, cells.count_alive_around(3, 1));
    assert_eq!(1, cells.count_alive_around(0, 2));
    assert_eq!(3, cells.count_alive_around(1, 2));
    assert_eq!(0, cells.count_alive_around(2, 2));
    assert_eq!(1, cells.count_alive_around(3, 2));
    assert_eq!(2, cells.count_alive_around(0, 4));
    assert_eq!(2, cells.count_alive_around(1, 4));
    assert_eq!(1, cells.count_alive_around(2, 4));
    assert_eq!(1, cells.count_alive_around(3, 4));
}

#[test]
fn test_cell_x() {
    let cells = CellsImpl::new(10, 5);

    assert_eq!(1, cells.cell_x(1));
    assert_eq!(2, cells.cell_x(12));
    assert_eq!(9, cells.cell_x(-1));
    assert_eq!(8, cells.cell_x(-12));
}

#[test]
fn test_cell_y() {
    let cells = CellsImpl::new(10, 5);

    assert_eq!(1, cells.cell_y(1));
    assert_eq!(2, cells.cell_y(7));
    assert_eq!(4, cells.cell_y(-1));
    assert_eq!(3, cells.cell_y(-7));
}
