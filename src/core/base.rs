/// RGB format
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pixel(pub u8, pub u8, pub u8);

/// x-y format
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub usize, pub usize);

impl std::ops::Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, (x_offset, y_offset): (i32, i32)) -> Point {
        Point(
            (self.0 as i32 + x_offset) as usize,
            (self.1 as i32 + y_offset) as usize,
        )
    }
}
