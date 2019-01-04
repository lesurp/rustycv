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
            let col_boundary = ((sq_r - x * x) as f64).sqrt() as i32;
            for y in -col_boundary..=col_boundary {
                frame.0[((point.0 as i32 + x) as usize, (point.1 as i32 + y) as usize, 0)] = color.0;
                frame.0[((point.0 as i32 + x) as usize, (point.1 as i32 + y) as usize, 1)] = color.1;
                frame.0[((point.0 as i32 + x) as usize, (point.1 as i32 + y) as usize, 2)] = color.2;
            }
        }
        Ok(())
    }
}
