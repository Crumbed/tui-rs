use std::rc::Rc;

use termion::color::Color;

pub trait Border {
    fn top_left(&self) -> char;
    fn top_right(&self) -> char;
    fn bottom_left(&self) -> char;
    fn bottom_right(&self) -> char;
    fn horizontal_left(&self) -> char;
    fn horizontal_right(&self) -> char;
    fn vertical_left(&self) -> char;
    fn vertical_right(&self) -> char;
}

impl Border for char {
    fn top_left(&self) -> char {
        *self
    }
    fn top_right(&self) -> char {
        *self
    }
    fn bottom_left(&self) -> char {
        *self
    }
    fn bottom_right(&self) -> char {
        *self
    }
    fn horizontal_left(&self) -> char {
        *self
    }
    fn horizontal_right(&self) -> char {
        *self
    }
    fn vertical_left(&self) -> char {
        *self
    }
    fn vertical_right(&self) -> char {
        *self
    }
}


#[derive(Clone)]
pub enum Align {
    Center,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Style {
    pub fg: Rc<dyn Color>,
    pub bg: Rc<dyn Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub align: Align,
    pub border: Rc<dyn Border>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Rc::new(termion::color::Reset),
            bg: Rc::new(termion::color::Reset),
            bold: false,
            italic: false,
            underline: false,
            align: Align::Left,
            border: Rc::new(' '),
        }
    }
}

//
