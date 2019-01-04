pub mod frame;

pub use self::frame::{Frame, RawFrame};

/// RGB format
#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

/// x-y format
pub struct Point(pub usize, pub usize);
