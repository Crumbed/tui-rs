use std::cmp::max;
use std::cmp::min;
use std::io;
use std::io::Stdout;
use std::io::Write;

use crate::widget::BorderChar;
use crate::widget::FillChar;
use crate::Vec2;

use termion::cursor::Goto;
use termion::raw::RawTerminal;
use termion::terminal_size;

pub enum DrawStyle {
    Full(FillChar),
    Border(FillChar, BorderChar),
}

pub trait ShapeDrawer {
    fn draw_square(&mut self, style: DrawStyle, pos: Vec2, size: Vec2) -> io::Result<()>;
    fn draw_line(&mut self, style: DrawStyle, pos1: Vec2, pos2: Vec2) -> io::Result<()>;
}

impl ShapeDrawer for RawTerminal<Stdout> {
    fn draw_square(&mut self, style: DrawStyle, pos: Vec2, size: Vec2) -> io::Result<()> {
        use DrawStyle::*;
        write!(self, "{}", Goto(pos.x(), pos.y()))?;
        let Vec2(p_x, p_y) = pos;
        let Vec2(w, h) = size;

        for y in 0_u16..h {
            for x in 0_u16..w {
                match style {
                    Full(c) => write!(self, "{}{}", Goto(x + p_x, y + p_y), c)?,

                    Border(f, b) => {
                        let c = match (x + p_x, y + p_y) {
                            (x, _) if x == p_x || x == w + p_x - 1 => b,
                            (_, y) if y == p_y || y == h + p_y - 1 => b,
                            _ => f,
                        };

                        write!(self, "{}{}", Goto(x + p_x, y + p_y), c)?;
                    }
                }
            }
        }

        write!(self, "{}", Goto(1, terminal_size().unwrap().1))?;
        Ok(())
    }

    fn draw_line(&mut self, style: DrawStyle, pos1: Vec2, pos2: Vec2) -> io::Result<()> {
        let Vec2(x1, y1) = pos1;
        let Vec2(x2, y2) = pos2;

        if y1 == y2 {
            if x1 > x2 {
                plot_line_horz(self, &style, Vec2(x2, y2), Vec2(x1, y1))?;
            } else {
                plot_line_horz(self, &style, Vec2(x1, y1), Vec2(x2, y2))?;
            }
        } else if x1 == x2 {
            if y1 > y2 {
                plot_line_vert(self, &style, Vec2(x2, y2), Vec2(x1, y1))?;
            } else {
                plot_line_vert(self, &style, Vec2(x1, y1), Vec2(x2, y2))?;
            }
        }

        if (y2 as i32 - y1 as i32).abs() < (x2 as i32 - x1 as i32).abs() {
            if x1 > x2 {
                plot_line_low(self, &style, Vec2(x2, y2), Vec2(x1, y1))?;
            } else {
                plot_line_low(self, &style, Vec2(x1, y1), Vec2(x2, y2))?;
            }
        } else {
            if y1 > y2 {
                plot_line_high(self, &style, Vec2(x2, y2), Vec2(x1, y1))?;
            } else {
                plot_line_high(self, &style, Vec2(x1, y1), Vec2(x2, y2))?;
            }
        }

        write!(self, "{}", Goto(1, terminal_size().unwrap().1))?;
        Ok(())
    }
}

fn plot(
    out: &mut RawTerminal<Stdout>,
    style: &DrawStyle,
    last_point: &mut Vec2,
    x: u16,
    y: u16,
) -> io::Result<()> {
    match style {
        DrawStyle::Full(c) => write!(out, "{}{}", Goto(x, y), c)?,

        DrawStyle::Border(f, b) => {
            let (w, h) = terminal_size()?;
            let mut output = format!("{}{}", Goto(x, y), f);

            if x - 1 > 0 && x - 1 != last_point.x() {
                output.push_str(&format!("{}{}", Goto(x - 1, y), b))
            }
            if x + 1 < w && x + 1 != last_point.x() {
                output.push_str(&format!("{}{}", Goto(x + 1, y), b))
            }

            if y - 1 > 0 && y - 1 != last_point.y() {
                output.push_str(&format!("{}{}", Goto(x, y - 1), b))
            }
            if y + 1 < h && y + 1 != last_point.y() {
                output.push_str(&format!("{}{}", Goto(x, y + 1), b))
            }

            last_point.0 = x;
            last_point.1 = y;
            write!(out, "{}", output)?;
        }
    }

    Ok(())
}

fn plot_line_low(
    out: &mut RawTerminal<Stdout>,
    style: &DrawStyle,
    pos1: Vec2,
    pos2: Vec2,
) -> io::Result<()> {
    let Vec2(x1, y1) = pos1;
    let Vec2(x2, y2) = pos2;

    let (dx, mut dy) = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32);
    let mut yi = 1i32;

    if dy < 0 {
        yi = -1;
        dy = -dy
    }
    let mut d = 2 * dy - dx;
    let mut y = y1;

    let mut last_point = Vec2(0, 0);
    for x in x1..=x2 {
        plot(out, style, &mut last_point, x, y)?;
        if d > 0 {
            y = (y as i32 + yi) as u16;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
    }

    Ok(())
}

fn plot_line_high(
    out: &mut RawTerminal<Stdout>,
    style: &DrawStyle,
    pos1: Vec2,
    pos2: Vec2,
) -> io::Result<()> {
    let Vec2(x1, y1) = pos1;
    let Vec2(x2, y2) = pos2;

    let (mut dx, dy) = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32);
    let mut xi = 1i32;

    if dx < 0 {
        xi = -1;
        dx = -dx
    }
    let mut d = 2 * dx - dy;
    let mut x = x1;

    let mut last_point = Vec2(0, 0);
    for y in y1..=y2 {
        plot(out, style, &mut last_point, x, y)?;
        if d > 0 {
            x = (x as i32 + xi) as u16;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }

    Ok(())
}

fn plot_line_horz(
    out: &mut RawTerminal<Stdout>,
    style: &DrawStyle,
    pos1: Vec2,
    pos2: Vec2,
) -> io::Result<()> {
    let Vec2(x1, y) = pos1;
    let x2 = pos2.x();

    let mut last_point = Vec2(0, y);
    for x in x1..=x2 {
        plot(out, style, &mut last_point, x, y)?;
        last_point.0 = x;
    }

    Ok(())
}

fn plot_line_vert(
    out: &mut RawTerminal<Stdout>,
    style: &DrawStyle,
    pos1: Vec2,
    pos2: Vec2,
) -> io::Result<()> {
    let Vec2(x, y1) = pos1;
    let y2 = pos2.y();

    let mut last_point = Vec2(x, 0);
    for y in y1..=y2 {
        plot(out, style, &mut last_point, x, y)?;
        last_point.1 = y;
    }

    Ok(())
}
