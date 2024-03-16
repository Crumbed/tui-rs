mod shapes;
mod utils;

use utils::Vec2;
use utils::Vec2Index;
use shapes::ShapeDrawer;
use shapes::DrawStyle;

use termion;
use termion::raw::IntoRawMode;

use std::io;
use std::io::stdout;
use std::io::Write;





fn main() -> io::Result<()> {
    let mut out = stdout().into_raw_mode().expect("WHY");
    write!(out, "{}", termion::clear::All)?;
    out.flush()?;
    //out.draw_square(DrawStyle::Border(' ', '#'), (2, 2), (10, 10))?;
    //out.draw_square(DrawStyle::Full('#'), (2, 2), (10, 10))?;
    //out.draw_line(DrawStyle::Border('#', ' '), (10, 8), (1, 1))?;
    out.draw_line(DrawStyle::Full('#'), (1, 1), (1, 10))?;
    out.flush()?;

    Ok(())
}





































