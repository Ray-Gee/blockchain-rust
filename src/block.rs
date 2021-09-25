use std::fmt::{ self, Debug, Formatter };

pub struct Block {
    pub index: u32,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block")
    }
}

impl Block {
}
