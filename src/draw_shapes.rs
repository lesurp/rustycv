use super::types::{Frame, Pixel, Point};

pub fn draw_dot(frame: &mut Frame, color: Pixel, center: Point, radius: usize) {
    let sq_r = radius as i32 * radius as i32;
    for x_offset in -(radius as i32)..=radius as i32 {
        let col_boundary = f64::from(sq_r - x_offset * x_offset).sqrt() as i32;
        for y_offset in -col_boundary..=col_boundary {
            let (x, y) = (center.0 as i32 + x_offset, center.1 as i32 + y_offset);
            if let Some(point) = frame.into_point(x, y) {
                frame[point] = color;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Frame, Pixel, Point};

    #[test]
    fn draw_ok() {
        let init_c = Pixel(255, 0, 0);
        let new_c = Pixel(0, 255, 0);

        let mut test_frame = Frame(ndarray::Array::from_shape_fn((123, 142), |(_, _)| init_c));

        let mut draw_some_dot = |x, y, r| {
            super::draw_dot(&mut test_frame, new_c, Point(x, y), r);
            let r = r as i32;
            let x = x as i32;
            let y = y as i32;

            for x_offset in -r..=r {
                let y_boundary = f64::from(r * r - x_offset * x_offset).sqrt() as i32;
                for y_loop in (y - y_boundary)..=(y + y_boundary) {
                    let this_x = x + x_offset;
                    let this_y = y_loop;
                    if let Some(point) = test_frame.into_point(this_x, this_y) {
                        assert_eq!(
                            test_frame[point],
                            new_c
                        );
                    }
                }
            }
        };

        // center in, touch border
        draw_some_dot(1, 1, 1);

        // radius = 0
        draw_some_dot(0, 0, 0);

        // nominal case
        draw_some_dot(61, 31, 7);

        // center in (border)
        draw_some_dot(122, 141, 10);
        draw_some_dot(1, 20, 5);

        // center out
        draw_some_dot(60, 150, 10);
        // TODO handle out-of-frame center
        //draw_some_dot(-1, 0, 7);
    }
}
