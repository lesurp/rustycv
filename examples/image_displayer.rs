use ndarray;
use rustycv::types::Frame;
use rustycv::image_displayer::ImageDisplay;

fn main() {
    let _ = env_logger::try_init();

    let mut id = ImageDisplay::new(69, 69, "LOL");

    let white = Frame(ndarray::Array::from_elem((69, 69, 3), 255u8));
    let red =
        Frame(ndarray::Array::from_shape_fn((69, 69, 3), |(_, _, c)| if c == 0 { 255u8 } else { 0 }));
    let green =
        Frame(ndarray::Array::from_shape_fn((69, 69, 3), |(_, _, c)| if c == 1 { 255u8 } else { 0 }));
    let blue =
        Frame(ndarray::Array::from_shape_fn((69, 69, 3), |(_, _, c)| if c == 2 { 255u8 } else { 0 }));

    let colors = [white, red, green, blue];
    for c in colors.iter().cycle() {
        if id.update(&c).is_err() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
