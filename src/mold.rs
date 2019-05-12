use cells::*;
use fonts::*;

pub struct Mold {
    pub font_size: usize,
    pub target: Cells,
}

impl Mold {
    pub fn new(font: Font, font_size: usize) -> Self {
        let mold_scale = font_size as f64 / font.val.len() as f64;
        let mut cells = Cells::new(font_size, font_size);
        cells.allocate(Cells::from_font(font), 0, 0, mold_scale);
        Mold {
            font_size: font_size,
            target: cells,
        }
    }

    pub fn from_char(c: char, font_size: usize) -> Self {
        match c {
            'A' => Mold::new(FONT_A, font_size),
            'B' => Mold::new(FONT_B, font_size),
            'C' => Mold::new(FONT_C, font_size),
            'D' => Mold::new(FONT_D, font_size),
            'E' => Mold::new(FONT_E, font_size),
            'F' => Mold::new(FONT_F, font_size),
            'G' => Mold::new(FONT_G, font_size),
            'H' => Mold::new(FONT_H, font_size),
            'I' => Mold::new(FONT_I, font_size),
            'J' => Mold::new(FONT_J, font_size),
            'K' => Mold::new(FONT_K, font_size),
            'L' => Mold::new(FONT_L, font_size),
            'N' => Mold::new(FONT_N, font_size),
            'M' => Mold::new(FONT_M, font_size),
            'O' => Mold::new(FONT_O, font_size),
            'P' => Mold::new(FONT_P, font_size),
            'Q' => Mold::new(FONT_Q, font_size),
            'R' => Mold::new(FONT_R, font_size),
            'S' => Mold::new(FONT_S, font_size),
            'T' => Mold::new(FONT_T, font_size),
            'U' => Mold::new(FONT_U, font_size),
            'V' => Mold::new(FONT_V, font_size),
            'W' => Mold::new(FONT_W, font_size),
            'X' => Mold::new(FONT_X, font_size),
            'Y' => Mold::new(FONT_Y, font_size),
            'Z' => Mold::new(FONT_Z, font_size),
            'a' => Mold::new(FONT_CASE_A, font_size),
            'b' => Mold::new(FONT_CASE_B, font_size),
            'c' => Mold::new(FONT_CASE_C, font_size),
            'd' => Mold::new(FONT_CASE_D, font_size),
            'e' => Mold::new(FONT_CASE_E, font_size),
            'f' => Mold::new(FONT_CASE_F, font_size),
            'g' => Mold::new(FONT_CASE_G, font_size),
            'h' => Mold::new(FONT_CASE_H, font_size),
            'i' => Mold::new(FONT_CASE_I, font_size),
            'j' => Mold::new(FONT_CASE_J, font_size),
            'k' => Mold::new(FONT_CASE_K, font_size),
            'l' => Mold::new(FONT_CASE_L, font_size),
            'n' => Mold::new(FONT_CASE_N, font_size),
            'm' => Mold::new(FONT_CASE_M, font_size),
            'o' => Mold::new(FONT_CASE_O, font_size),
            'p' => Mold::new(FONT_CASE_P, font_size),
            'q' => Mold::new(FONT_CASE_Q, font_size),
            'r' => Mold::new(FONT_CASE_R, font_size),
            's' => Mold::new(FONT_CASE_S, font_size),
            't' => Mold::new(FONT_CASE_T, font_size),
            'u' => Mold::new(FONT_CASE_U, font_size),
            'v' => Mold::new(FONT_CASE_V, font_size),
            'w' => Mold::new(FONT_CASE_W, font_size),
            'x' => Mold::new(FONT_CASE_X, font_size),
            'y' => Mold::new(FONT_CASE_Y, font_size),
            'z' => Mold::new(FONT_CASE_Z, font_size),
            '1' => Mold::new(FONT_1, font_size),
            '2' => Mold::new(FONT_2, font_size),
            '3' => Mold::new(FONT_3, font_size),
            '4' => Mold::new(FONT_4, font_size),
            '5' => Mold::new(FONT_5, font_size),
            '6' => Mold::new(FONT_6, font_size),
            '7' => Mold::new(FONT_7, font_size),
            '8' => Mold::new(FONT_8, font_size),
            '9' => Mold::new(FONT_9, font_size),
            '0' => Mold::new(FONT_0, font_size),
            _ => Mold::new(FONT_EMPTY, font_size),
        }
    }

    pub fn intersect(&self, cells: &Cells) -> u64 {
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

    pub fn difference(&self, cells: &Cells) -> u64 {
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

    assert_eq!(20, mold.intersect(&cells));
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

    assert_eq!(24, mold.difference(&cells));
}
