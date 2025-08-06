use std::fmt::{ Display, Debug };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Speed(pub u8);

impl Speed {
    pub const GLACIAL: Speed = Speed(10);
    pub const SLOW:   Speed = Speed(30);
    pub const MEDIUM: Speed = Speed(60); //16.66 ms per tick
    pub const FAST:   Speed = Speed(100);
    pub const ULTRA:  Speed = Speed(120);
    pub const HYPER:  Speed = Speed(240);
    #[inline] pub fn new(v: u8) -> Speed { Speed(v) }
}

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

