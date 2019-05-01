use cells::*;
use rand::prelude::*;
use wasm_bindgen::JsValue;

pub struct GameOfLife {
    width: f64,
    height: f64,
    origin_x: f64,
    origin_y: f64,
    cell_length: f64,
    cells: CellsImpl,
}

impl GameOfLife {
    pub fn new(size_x: usize, size_y: usize) -> GameOfLife {
        GameOfLife {
            width: 1000.0,
            height: 500.0,
            origin_x: 20.0,
            origin_y: 20.0,
            cell_length: 10.0,
            cells: CellsImpl::new(size_x, size_y),
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.cells.size_x {
            for y in 0..self.cells.size_y {
                let r: f64 = rng.gen();
                if r < 0.3 {
                    self.cells.make_alive(x, y);
                }
            }
        }
    }

    pub fn evolve(&mut self) {
        let mut new_cells = self.cells.replicate();

        for x in 0..self.cells.size_x {
            for y in 0..self.cells.size_y {
                let c = self.cells.count_alive_around(x, y);
                if self.cells.is_alive(x, y) && c == 2 || c == 3 {
                    new_cells.make_alive(x, y);
                }
            }
        }

        self.cells = new_cells;
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from_str("white"));
        context.fill_rect(0.0, 0.0, self.width, self.height);

        context.set_fill_style(&JsValue::from_str("black"));

        context.begin_path();
        self.draw_grid(&context);
        context.stroke();

        self.draw_cells(&context);
    }

    fn draw_grid(&self, context: &web_sys::CanvasRenderingContext2d) {
        let to_x = self.origin_x + self.cell_length * self.cells.size_x as f64;
        let to_y = self.origin_y + self.cell_length * self.cells.size_y as f64;

        for x in 0..(self.cells.size_x + 1) {
            let from_x = self.origin_x + self.cell_length * x as f64;
            context.move_to(from_x, self.origin_y);
            context.line_to(from_x, to_y);
        }

        for y in 0..(self.cells.size_y + 1) {
            let from_y = self.origin_y + self.cell_length * y as f64;
            context.move_to(self.origin_x, from_y);
            context.line_to(to_x, from_y);
        }
    }

    pub fn draw_cells(&self, context : &web_sys::CanvasRenderingContext2d) {
        for x in 0..(self.cells.size_x) {
           for y in 0..(self.cells.size_y) {
               if self.cells.is_alive(x, y) {
                   let upper_x = self.origin_x + self.cell_length * x as f64;
                   let upper_y = self.origin_y + self.cell_length * y as f64;
                   context.fill_rect(upper_x, upper_y, self.cell_length, self.cell_length);
               }
           }
        }
    }
}
