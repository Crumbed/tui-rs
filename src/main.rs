mod shapes;
mod utils;
mod div;

use div::div;
use utils::Vec2;
use shapes::ShapeDrawer;
use shapes::DrawStyle;

use termion;
use termion::raw::IntoRawMode;

use std::io;
use std::io::stdout;
use std::io::Write;





fn main() -> io::Result<()> {
    //let mut out = stdout().into_raw_mode().expect("WHY");
    //write!(out, "{}", termion::clear::All)?;

    let test = format!("{}Hello", termion::color::Fg(termion::color::Red));
    println!("{test}");
    println!("{:?}", test);
    println!("{:?}", test.chars().collect::<Vec<char>>());
    println!("{}", test.len());
    
    Ok(())
}





































