/// RGB format
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pixel(pub u8, pub u8, pub u8);

/// x-y format
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub usize, pub usize);
