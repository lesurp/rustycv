use ndarray;
use rustycv::image_displayer::ImageDisplay;
use rustycv::types::{Frame, Pixel};

fn main() {
    let _ = env_logger::try_init();

    let mut id = ImageDisplay::new(69, 69, "LOL");

    let white = Frame(ndarray::Array::from_elem((69, 69), Pixel(255, 255, 255)));
    let red = Frame(ndarray::Array::from_shape_fn((69, 69), |(_, _)| {
        Pixel(255, 0, 0)
    }));
    let green = Frame(ndarray::Array::from_shape_fn((69, 69), |(_, _)| {
        Pixel(0, 255, 0)
    }));
    let blue = Frame(ndarray::Array::from_shape_fn((69, 69), |(_, _)| {
        Pixel(0, 0, 255)
    }));

    let colors = [white, red, green, blue];
    for c in colors.iter().cycle() {
        if id.update(&c).is_err() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
