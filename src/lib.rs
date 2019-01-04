#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
extern crate image;
extern crate ndarray;

pub mod types;
pub mod image_displayer;
pub mod image_loader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
