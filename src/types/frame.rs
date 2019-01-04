pub struct Frame(pub ndarray::Array3<u8>);

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

