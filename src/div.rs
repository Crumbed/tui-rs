








use crate::Vec2;
use std::io;





pub trait Div {
    fn style(self, style: Style) -> Box<dyn Div>;
    fn pos(self, pos: Vec2) -> Box<dyn Div>;
    fn size(self, size: Vec2) -> Box<dyn Div>;
    fn get_buffer(&mut self) -> &mut [char];
    fn get_size(&self) -> Vec2;
    fn get_pos(&self) -> Vec2;
    fn get_opts(&self) -> DivOpts;

    fn render(&mut self) -> io::Result<Box<[Box<[char]>]>>;
}

#[derive(Clone, Copy)]
pub struct DivOpts {
    pub align: Align,
    pub size: Vec2,
    pub pos: Vec2,
    pub border: Option<BorderChar>,
    pub fill: Option<FillChar>,
    pub float: Float, 
    pub pos_type: PositionType, 
}

impl Default for DivOpts {
    fn default() -> Self {
        DivOpts {
            align: Align::Left,
            size: Vec2(0, 0),
            pos: Vec2(0, 0),
            border: None,
            fill: None,
            float: Float::None,
            pos_type: PositionType::Relative, 
        }
    }
}

#[derive(Clone, Copy)]
pub enum Align {
    Center, 
    Left, 
    Right
}

#[derive(Clone, Copy)]
pub enum Float {
    Left, 
    Right, 
    None
}

#[derive(Clone, Copy, PartialEq)]
pub enum PositionType {
    Absolute, 
    Relative
}

pub type FillChar = char;
pub type BorderChar = char;
pub enum Style {
    Align(Align),
    Border(BorderChar), 
    Fill(FillChar), 
    Float(Float)
}


pub struct RawDiv {
    pub opts: DivOpts,
    pub body: Vec<Box<dyn Div>>,
    pub buffer: Vec<char>
}

impl Div for RawDiv {
    fn style(mut self, style: Style) -> Box<dyn Div> {
        match style {
            Style::Align(align) => { self.opts.align = align; },
            Style::Border(border) => { self.opts.border = Some(border); },
            Style::Fill(fill) => { self.opts.fill = Some(fill); },
            Style::Float(float) => { self.opts.float = float; }
        }

        return Box::new(self);
    }

    fn pos(mut self, pos: Vec2) -> Box<dyn Div> {
        self.opts.pos = pos;
        return Box::new(self);
    }

    fn size(mut self, size: Vec2) -> Box<dyn Div> {
        self.opts.size = size;
        return Box::new(self);
    }


    fn get_buffer(&mut self) -> &mut [char] { &mut self.buffer }
    fn get_pos(&self) -> Vec2 { self.opts.pos }
    fn get_size(&self) -> Vec2 { self.opts.size }
    fn get_opts(&self) -> DivOpts { self.opts }

    fn render(&mut self) -> io::Result<Box<[Box<[char]>]>> {
        let mut buffer = vec![
            vec![' '; self.opts.size.x().into()]; 
            self.opts.size.y().into()
        ];

        for (i, child) in self.body.iter_mut().enumerate() {
            let c_size = child.get_size();
            //let c_pos = child.get_pos();
            //let c_pos_type = child.get_opts().pos_type;

            if c_size >= self.opts.size {

            }
        }

        let buffer = buffer.iter_mut()
            .map(|x| x.clone().into_boxed_slice())
            .collect::<Vec<Box<[char]>>>()
            .into_boxed_slice();

        return Ok(buffer);
    }
}

pub fn div(body: Vec<Box<dyn Div>>) -> Box<RawDiv> {
    return Box::new(RawDiv {
        opts: DivOpts::default(),
        body, 
        buffer: vec![' '; 0]
    });
}



































