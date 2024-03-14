use std::{io::{Stdout, Write, stdout}, os::fd::AsFd};

use termion::raw::{RawTerminal, IntoRawMode};














pub struct Term<T: AsFd + Write>(pub RawTerminal<T>);

impl<T: AsFd + Write> Term<T> {
    pub fn new() -> Term<Stdout> {
        let stdout = stdout().into_raw_mode().expect("Failed to enter raw terminal");
        return Term(stdout);
    }


}





















