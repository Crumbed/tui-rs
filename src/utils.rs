






pub type Vec2 = (u16, u16);


pub trait Vec2Index {
    fn x(&self) -> u16; 
    fn y(&self) -> u16;
}

impl Vec2Index for Vec2 {
    fn x(&self) -> u16 { self.0 }
    fn y(&self) -> u16 { self.1 }
}


