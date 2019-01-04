#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
extern crate image;
extern crate ndarray;

pub mod image_displayer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
