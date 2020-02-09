/// Generic way
pub enum GenericMat {
    F32_1(Mat<[f32; 1]>),
    F32_3(Mat<[f32; 3]>),

    F64_1(Mat<[f64; 1]>),
    F64_3(Mat<[f64; 3]>),

    U8_1(Mat<[u8; 1]>),
    U8_3(Mat<[u8; 3]>),

    I32_1(Mat<[i32; 1]>),
    I32_3(Mat<[i32; 3]>),
}

///////////////////////// POINT
pub struct Point(usize, usize);
impl Point {
    pub fn from_xy(x: usize, y: usize) -> Self {
        Point(y, x)
    }

    pub fn from_row_col(i: usize, j: usize) -> Self {
        Point(i, j)
    }

    pub fn i(&self) -> usize {
        self.0
    }

    pub fn j(&self) -> usize {
        self.1
    }

    pub fn x(&self) -> usize {
        self.j()
    }

    pub fn y(&self) -> usize {
        self.i()
    }
}

/////////////////////////// STORED MAT TYPE
pub trait Pixel {
    type Type;
    fn channels() -> usize;
}

impl<T> Pixel for [T; 3] {
    type Type = T;
    fn channels() -> usize {
        3
    }
}

impl<T> Pixel for [T; 1] {
    type Type = T;
    fn channels() -> usize {
        1
    }
}

/////////////////////////// MAT
pub struct Mat<Px: Pixel> {
    data: ndarray::Array2<Px>,
}

impl<Px: Pixel> Mat<Px> {
    pub fn w(&self) -> usize {
        self.data.len_of(ndarray::Axis(0))
    }

    pub fn h(&self) -> usize {
        self.data.len_of(ndarray::Axis(1))
    }
}
