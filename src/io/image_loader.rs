use crate::core::{Mat, Pixel, GenericMat};
use image::GenericImageView;

#[derive(Debug, PartialEq)]
pub enum LoadImageErr {
    FileNotFound,
    Dunno,
}

pub fn image_buffer_to_mat<P, Container>(m: image::ImageBuffer<P, Container>) -> GenericMat 
where
    P: image::Pixel + 'static,
    P::Subpixel: 'static,
    Container: std::ops::Deref<Target = [P::Subpixel]>, {
}

pub fn load_image(path: &std::path::Path) -> Result<GenericMat, LoadImageErr> {
    use image::DynamicImage;
    let image = image::open(path).map_err(|_| LoadImageErr::FileNotFound)?;
    let dimensions = image.dimensions();
    match image {
        DynamicImage::ImageLuma8(m) => {},
    }
    let raw_image = rgb_image.raw_pixels();
    let u8_ndarray : ndarray::Array3<u8> = ndarray::ArrayBase::from_shape_vec(
        (dimensions.0 as usize, dimensions.1 as usize, 3),
        raw_image,
    )
    .map_err(|_| LoadImageErr::Dunno)?;
    let pixel_ndarray = u8_ndarray.map_axis(ndarray::Axis(2), |pixels| {
        Pixel(pixels[0], pixels[1], pixels[2])
    });
    Ok(Frame(pixel_ndarray))
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
