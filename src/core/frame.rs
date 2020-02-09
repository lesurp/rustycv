use super::base::{Pixel, Point};

pub type RawFrame = ndarray::Array2<Pixel>;

pub struct Frame(pub RawFrame);

impl Frame {
    pub fn w(&self) -> usize {
        self.0.len_of(ndarray::Axis(0))
    }

    pub fn h(&self) -> usize {
        self.0.len_of(ndarray::Axis(1))
    }

    pub fn center(&self) -> (usize, usize) {
        (self.w() >> 1, self.h() >> 1)
    }

    /// We cannot use a Pixel value for the image crates (used to display the frames),
    /// meaning we either need to stick with an Array3<u8> (instead of an Array2<Pixel>)
    /// However, right now, we are copying the data in the displayer.
    /// Meaning we might as well do the conversion manually - which is what this function does
    pub fn to_flat_vec(&self) -> Vec<u8> {
        let mut flat = Vec::with_capacity(3 * self.h() * self.w());
        for pixel in self.0.iter() {
            flat.push(pixel.0);
            flat.push(pixel.1);
            flat.push(pixel.2);
        }
        flat
    }

    pub fn is_in(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.w() as i32 && y >= 0 && y < self.h() as i32
    }

    pub fn into_point(&self, x: i32, y: i32) -> Option<Point> {
        if x >= 0 && x < self.w() as i32 && y >= 0 && y < self.h() as i32 {
            Some(Point(x as usize, y as usize))
        } else {
            None
        }
    }
}

impl std::ops::Index<Point> for Frame {
    type Output = Pixel;

    fn index(&self, p: Point) -> &Pixel {
        &self.0[(p.0, p.1)]
    }
}

impl std::ops::IndexMut<Point> for Frame {
    fn index_mut(&mut self, p: Point) -> &mut Pixel {
        &mut self.0[(p.0, p.1)]
    }
}
