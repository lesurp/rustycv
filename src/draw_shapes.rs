use super::types;

pub fn draw_dot(
    frame: &mut types::Frame,
    color: types::Color,
    point: types::Point,
    radius: usize,
) -> Result<(), ()> {
    if point.0 < radius
        || point.1 < radius
        || point.0 + radius >= frame.w()
        || point.1 + radius >= frame.h()
    {
        Err(())
    } else {
        let sq_r = radius as i32 * radius as i32;
        for x in -(radius as i32)..=radius as i32 {
            let col_boundary = f64::from(sq_r - x * x).sqrt() as i32;
            for y in -col_boundary..=col_boundary {
                frame.0[(
                    (point.0 as i32 + x) as usize,
                    (point.1 as i32 + y) as usize,
                    0,
                )] = color.0;
                frame.0[(
                    (point.0 as i32 + x) as usize,
                    (point.1 as i32 + y) as usize,
                    1,
                )] = color.1;
                frame.0[(
                    (point.0 as i32 + x) as usize,
                    (point.1 as i32 + y) as usize,
                    2,
                )] = color.2;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::types::{Color, Frame, Point};

    #[test]
    fn draw_ok() {
        let init_c = Color(255, 0, 0);
        let new_c = Color(0, 255, 0);

        let mut test_frame = Frame(ndarray::Array::from_shape_fn(
            (123, 42, 3),
            |(_, _, c)| match c {
                0 => init_c.0,
                1 => init_c.1,
                2 => init_c.2,
                _ => unreachable!(),
            },
        ));

        let mut draw_some_dot = |x, y, r| {
            assert!(super::draw_dot(&mut test_frame, new_c, Point(x, y), r).is_ok());
            let r = r as i32;
            let x = x as i32;
            let y = y as i32;

            for x_offset in -r..=r {
                let y_boundary = f64::from(r * r - x_offset * x_offset).sqrt() as i32;
                for y_loop in (y - y_boundary)..=(y + y_boundary) {
                    assert_eq!(test_frame.0[((x + x_offset) as usize, y_loop as usize, 0)], new_c.0);
                    assert_eq!(test_frame.0[((x + x_offset) as usize, y_loop as usize, 1)], new_c.1);
                    assert_eq!(test_frame.0[((x + x_offset) as usize, y_loop as usize, 2)], new_c.2);
                }
            }
        };

        draw_some_dot(1, 1, 1);
        draw_some_dot(0, 0, 0);
        draw_some_dot(122, 41, 0);
        draw_some_dot(121, 40, 1);
        draw_some_dot(61, 31, 7);
    }

    #[test]
    fn draw_err() {
        let init_c = Color(255, 0, 0);
        let new_c = Color(0, 255, 0);

        let mut test_frame = Frame(ndarray::Array::from_shape_fn(
            (123, 42, 3),
            |(_, _, c)| match c {
                0 => init_c.0,
                1 => init_c.1,
                2 => init_c.2,
                _ => unreachable!(),
            },
        ));

        let mut draw_some_dot = |x, y, r| {
            assert!(super::draw_dot(&mut test_frame, new_c, Point(x, y), r).is_err());
        };

        draw_some_dot(1, 1, 2);
        draw_some_dot(0, 0, 1);
        draw_some_dot(122, 41, 1);
        draw_some_dot(121, 40, 2);
        draw_some_dot(61, 31, 13);
    }
}
