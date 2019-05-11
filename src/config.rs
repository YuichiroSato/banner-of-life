use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Config {
    pub cell_size: usize,
}

#[wasm_bindgen]
impl Config {
    pub fn new() -> Config {
        Config {
            cell_size: 4
        }
    }

    #[wasm_bindgen(js_name = setCellSize)]
    pub fn set_cell_size(&mut self, cell_size: usize) {
        self.cell_size = cell_size;
    }
}

