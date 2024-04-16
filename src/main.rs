mod shapes;
mod style;
mod utils;
mod widget;

use shapes::DrawStyle;
use shapes::ShapeDrawer;
use utils::Vec2;

use widget::TWidget;
use widget::widget;

use termion;
use termion::raw::IntoRawMode;

use std::io;
use std::io::stdout;
use std::io::Write;
use std::rc::Rc;

fn main() -> io::Result<()> {
    //let mut out = stdout().into_raw_mode().expect("WHY");
    //write!(out, "{}", termion::clear::All)?;

    let main = widget(vec![
        widget(vec![]),
        widget(vec![]),
        widget(vec![])
    ]).pos(Vec2(0, 0))
    .size(Vec2(10, 10));


    Ok(())
}
