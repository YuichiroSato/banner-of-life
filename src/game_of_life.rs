use cells::*;
use config::*;
use wasm_bindgen::JsValue;
use evolve::*;

pub struct GameOfLife {
    width: f64,
    height: f64,
    cell_length: f64,
    cells: Cells,
}

impl GameOfLife {
    pub fn new(width: f64, height: f64, config: Config) -> GameOfLife {
        let size_x = (width as f64 / config.cell_size as f64) as usize;
        let size_y = (height as f64 / config.cell_size as f64) as usize;
        GameOfLife {
            width: width,
            height: height,
            cell_length: config.cell_size as f64,
            cells: Cells::new(size_x, size_y),
        }
    }

    pub fn evolve(&mut self) {
        self.cells = next(&self.cells);
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
        let to_x = self.cell_length * self.cells.size_x as f64;
        let to_y = self.cell_length * self.cells.size_y as f64;

        for x in 0..(self.cells.size_x + 1) {
            let from_x = self.cell_length * x as f64;
            context.move_to(from_x, 0.0);
            context.line_to(from_x, to_y);
        }

        for y in 0..(self.cells.size_y + 1) {
            let from_y = self.cell_length * y as f64;
            context.move_to(0.0, from_y);
            context.line_to(to_x, from_y);
        }
    }

    pub fn draw_cells(&self, context : &web_sys::CanvasRenderingContext2d) {
        for x in 0..(self.cells.size_x) {
           for y in 0..(self.cells.size_y) {
               if self.cells.is_alive(x, y) {
                   let upper_x = self.cell_length * x as f64;
                   let upper_y = self.cell_length * y as f64;
                   context.fill_rect(upper_x, upper_y, self.cell_length, self.cell_length);
               }
           }
        }
    }
}
