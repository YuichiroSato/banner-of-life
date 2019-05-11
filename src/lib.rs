extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod cells;
mod compound;
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
}

#[wasm_bindgen]
impl Banner {
    pub fn new(canvas_id: &str) -> Banner {
        Banner {
            canvas_id: canvas_id.to_string(),
            game_of_life: GameOfLife::new(100, 30),
        }
    }

    pub fn initialize(&mut self) {
        self.game_of_life.randomize();
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

        self.game_of_life.draw(&context);
    }
}
