pub struct Config {
    pub cell_size: usize,
    pub font_size: usize,
    pub background_color: String,
    pub cell_color: String,
    pub grid_color: String,
    pub mold_color: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            cell_size: 4,
            font_size: 40,
            background_color: "white".to_string(),
            cell_color: "black".to_string(),
            grid_color: "black".to_string(),
            mold_color: "black".to_string(),
        }
    }
}

