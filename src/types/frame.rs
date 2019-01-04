//#[derive(Clone, Copy)]
//pub struct Pixel(u8, u8, u8);

pub type RawFrame = ndarray::Array3<u8>;

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
}

