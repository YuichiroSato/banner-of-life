pub trait Cells {
    fn new(size_x: usize, size_y: usize) -> Self;
    fn replicate(&self) -> Self;
    fn make_alive(&mut self, x: usize, y: usize);
    fn make_dead(&mut self, x: usize, y: usize);
    fn is_alive(&self, x: usize, y: usize) -> bool;
    fn count_alive_around(&self, x: usize, y: usize) -> i32;
}
