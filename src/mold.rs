use cells::*;
use fonts::*;

pub struct Mold {
    pub target: Cells,
}

impl Mold {
    pub fn new(font: Font, font_size: usize) -> Self {
        let mold_scale = font_size as f64 / font.val.len() as f64;
        let mut cells = Cells::new(font_size, font_size);
        cells.allocate(Cells::from_font(font), 0, 0, mold_scale);
        Mold {
            target: cells,
        }
    }

    pub fn intersect(&self, cells: Cells) -> u64 {
        let mut count = 0;

        for x in 0..self.target.size_x {
            for y in 0..self.target.size_y {
                if self.target.is_alive(x, y) && cells.is_alive(x, y) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn difference(&self, cells: Cells) -> u64 {
        let mut count = 0;

        for x in 0..self.target.size_x {
            for y in 0..self.target.size_y {
                if !self.target.is_alive(x, y) && cells.is_alive(x, y) {
                    count += 1;
                }
            }
        }

        count
    }
}

#[test]
fn test_intersect() {
    let test_font = Font {
        val : &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
            &[0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    };
    let test_font2 = Font {
        val : &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    };

    let mold = Mold::new(test_font, 20);
    let mut cells = Cells::new(20, 20);
    cells.allocate(Cells::from_font(test_font2), 0, 0, 2.0);

    assert_eq!(20, mold.intersect(cells));
}

#[test]
fn test_difference() {
    let test_font = Font {
        val : &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
            &[0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    };
    let test_font2 = Font {
        val : &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    };

    let mold = Mold::new(test_font, 20);
    let mut cells = Cells::new(20, 20);
    cells.allocate(Cells::from_font(test_font2), 0, 0, 2.0);

    assert_eq!(24, mold.difference(cells));
}
