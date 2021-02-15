#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red = 1,
    Orange = 2,
    Green = 3,
    Blue = 4,
    Yellow = 5,
    White = 6,
}

#[derive(Debug)]
pub struct Cube {
    pub height: u8,
    pub width: u8,
    pub depth: u8,
    pub front: Side,
    pub back: Side,
    pub right: Side,
    pub left: Side,
    pub top: Side,
    pub bottom: Side,
}

#[derive(Debug)]
pub struct Side {
    pub height: u8,
    pub width: u8,
    pub color: Color,
    pub pieces: Vec<Vec<Color>>,
}

impl Side {
    pub fn make_side(h: u8, w: u8, c: Color) -> Side {
        let p = vec![vec![c; w as usize]; h as usize];
        Side {
            height: h,
            width: w,
            color: c,
            pieces: p,
        }
    }

    pub fn to_str(&self) -> String {
        let mut out_string = "".to_string();
        for row in &self.pieces[..] {
            let mut row_string = "".to_string();
            for color in row {
                let temp_str =  match color {
                    Color::Red => "R",
                    Color::Yellow => "Y",
                    Color::Orange => "O",
                    Color::White => "W",
                    Color::Green => "G",
                    Color::Blue => "B",
                };
                row_string = format!("{} {}", row_string, temp_str)
            }
            out_string = format!("{}\n{}", out_string, row_string)
        }
        out_string
    }
}


impl Cube {
    pub fn make_normal(size: u8) -> Cube {
        Cube {
            height: size,
            width: size,
            depth: size,
            front: Side::make_side(size, size, Color::Red),
            back: Side::make_side(size, size, Color::Orange),
            left: Side::make_side(size, size, Color::Green),
            right: Side::make_side(size, size, Color::Blue),
            top: Side::make_side(size, size, Color::White),
            bottom: Side::make_side(size, size, Color::Yellow),
        }
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nTop: {} \nFront: {} \nLeft: {} \nRight: {} \nBottom: {} \nBack: {}",
               self.top.to_str(), self.front.to_str(), self.left.to_str(), self.right.to_str(),
               self.bottom.to_str(), self.back.to_str(),)
    }
}

