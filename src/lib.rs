extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod cells;
mod compound;
mod config;
mod evolve;
mod fonts;
mod game_of_life;
mod mold;
mod optimizer;
mod primes;
mod randomizer;
mod utils;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use game_of_life::GameOfLife;
use config::*;
use mold::*;
use optimizer::*;
use randomizer::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub struct Banner {
    canvas_id: String,
    game_of_life: GameOfLife,
    config: Config,
}

#[wasm_bindgen]
impl Banner {
    pub fn new(canvas_id: &str) -> Banner {
        let config = Config::new();
        let (width, height) = Banner::get_canvas_size(canvas_id);
        Banner {
            canvas_id: canvas_id.to_string(),
            game_of_life: GameOfLife::new(width, height, config.cell_size),
            config: config,
        }
    }

    fn get_canvas_size(canvas_id: &str) -> (f64, f64) {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&canvas_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        (canvas.width() as f64, canvas.height() as f64)
    }

    #[wasm_bindgen(js_name = setCellSize)]
    pub fn set_cell_size(&mut self, cell_size: usize) {
        self.config.cell_size = cell_size;
        let (width, height) = Banner::get_canvas_size(self.canvas_id.as_str());
        self.game_of_life =  GameOfLife::new(width, height, cell_size);
    }

    #[wasm_bindgen(js_name = setFontSize)]
    pub fn set_font_size(&mut self, font_size: usize) {
        self.config.font_size = font_size;
    }

    #[wasm_bindgen(js_name = setBackgroundColor)]
    pub fn set_background_color(&mut self, background_color: &str) {
        self.config.background_color = background_color.to_string();
    }

    #[wasm_bindgen(js_name = setCellColor)]
    pub fn set_cell_color(&mut self, cell_color: &str) {
        self.config.cell_color = cell_color.to_string();
    }

    #[wasm_bindgen(js_name = setGridColor)]
    pub fn set_grid_color(&mut self, grid_color: &str) {
        self.config.grid_color = grid_color.to_string();
    }

    pub fn render(&mut self, text: &str) {
       let rand = Rand::new();
       let mut optimizer = GradientDescent {
            randomizer: rand,
            n: 5,
        };
        let cs: Vec<char> = text.chars().collect();
        let mut i = 0;
        for c in cs {
            let mold = Mold::from_char(c, self.config.font_size);
            let pattern = optimizer.optimize(mold);
            let cells = pattern.to_cells();
            self.game_of_life.clear();
            self.game_of_life.allocate(cells, 5 + i * self.config.font_size, 5);
            i += 1;
        }
    }

    pub fn tick(&mut self) {
        self.game_of_life.evolve();
        self.draw();
    }

    pub fn draw(&self) {
        let context = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&*self.canvas_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        self.game_of_life.draw(&context, &self.config);
    }
}
