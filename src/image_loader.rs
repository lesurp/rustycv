use image;
use image::GenericImageView;
use ndarray;

#[derive(Debug, PartialEq)]
pub enum LoadImageErr {
    FileNotFound,
    Dunno,
}

pub fn load_image(path: &std::path::Path) -> Result<ndarray::Array3<u8>, LoadImageErr> {
    let rgb_image = image::open(path).map_err(|_| LoadImageErr::FileNotFound)?;
    let dimensions = rgb_image.dimensions();
    let raw_image = rgb_image.raw_pixels();
    Ok(ndarray::ArrayBase::from_shape_vec(
        (dimensions.0 as usize, dimensions.1 as usize, 3),
        raw_image,
    )
    .map_err(|_| LoadImageErr::Dunno)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_lena() {
        let assets = find_folder::Search::ParentsThenKids(2, 1)
            .for_folder("resources")
            .unwrap();
        let path = assets.join("lena.png");

        let r = load_image(&path);
        assert!(r.is_ok());
    }

    #[test]
    fn load_rubbish() {
        let r = load_image(std::path::Path::new("/ewrgnoirn/soeboewrb/werjognoeib"));
        assert!(r.is_err());
        match r {
            Ok(_) => unreachable!(),
            Err(e) => assert_eq!(e, LoadImageErr::FileNotFound),
        }
    }
}
