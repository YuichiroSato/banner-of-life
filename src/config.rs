pub struct Config {
    pub cell_size: usize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            cell_size: 4
        }
    }
}

