use find_folder;
use ndarray;
use rustycv::image_displayer::ImageDisplay;
use rustycv::image_loader::*;

fn main() {
    let _ = env_logger::try_init();

    let assets = find_folder::Search::ParentsThenKids(2, 1)
        .for_folder("resources")
        .unwrap();
    let path = assets.join("lena.png");

    let image = load_image(&path).unwrap();
    let width = image.len_of(ndarray::Axis(0)) as u32;
    let height = image.len_of(ndarray::Axis(1)) as u32;

    // Note regarding the size:
    // - it does NOT have to match the image_displayer's
    // - if it doesn't, it's gonna be shrinked / expanded
    // - if the size is too big, you'll only get a black window FIXME
    let mut id = ImageDisplay::new(width, height, "LOL");
    let _ = id.update(&image);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
