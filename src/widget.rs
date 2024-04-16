use crate::{style::Style, Vec2};
use std::{fmt::Display, io, rc::Rc};

pub trait TWidget {
    fn style(self, style: Style) -> Self;
    fn pos(self, pos: Vec2) -> Self;
    fn size(self, size: Vec2) -> Self;
    fn get_buffer(&mut self) -> &mut [char];
    fn get_size(&self) -> Vec2;
    fn get_pos(&self) -> Vec2;
    fn get_opts(&self) -> &WidgetOpts;
}

#[derive(Clone)]
pub struct WidgetOpts {
    pub pos: Vec2,
    pub size: Vec2,
    pub style: Style,
}

impl Default for WidgetOpts {
    fn default() -> Self {
        WidgetOpts {
            size: Vec2(0, 0),
            pos: Vec2(0, 0),
            style: Style::default(),
        }
    }
}

pub type FillChar = char;
pub type BorderChar = char;

#[derive(Clone)]
pub struct Cell {
    pub char: char,
    pub style: Rc<dyn Display>,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        Cell {
            char: c,
            style: Rc::new(String::new()),
        }
    }
}

pub struct Widget {
    pub opts: WidgetOpts,
    pub body: Vec<Box<dyn TWidget>>,
    pub buffer: Vec<char>,
}

impl TWidget for Widget {
    fn style(mut self, style: Style) -> Self {
        self.opts.style = style;
        return self;
    }

    fn pos(mut self, pos: Vec2) -> Self {
        self.opts.pos = pos;
        return self;
    }

    fn size(mut self, size: Vec2) -> Self {
        self.opts.size = size;
        return self;
    }

    fn get_buffer(&mut self) -> &mut [char] {
        &mut self.buffer
    }
    fn get_pos(&self) -> Vec2 {
        self.opts.pos
    }
    fn get_size(&self) -> Vec2 {
        self.opts.size
    }
    fn get_opts(&self) -> &WidgetOpts {
        &self.opts
    }
}

pub fn widget(body: Vec<Box<dyn TWidget>>) -> Box<Widget> {
    return Box::new(Widget {
        opts: WidgetOpts::default(),
        body,
        buffer: vec![' '.into(); 0],
    });
}
