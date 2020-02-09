use super::types::{Frame, Point};

struct Orb;

///
/// This is what we describe as a patch:
///        x
///        _   x
///        _   x   x
///        o   _   _   x
///
/// o is the patch center, _ are ignored pixels, x are processed pixels.
fn compute_orb_descriptors(f: &Frame) -> Vec<Orb> {
    let bresenham_offsets = [
        // 12 to 2
        [0, 3],
        [1, 2],
        [1, 1],
        [2, 1],
        // 3 to 5
        [3, 0],
        [2, -1],
        [1, -1],
        [1, -2],
        // 6 to 9
        [0, -3],
        [-1, -2],
        [-1, -1],
        [-2, -1],
        // 9 to 12
        [-3, 0],
        [-2, 1],
        [-1, 1],
        [-1, 2],
    ];

    const RADIUS: usize = 3;

    let x_boundaries = Point(RADIUS, f.w()  - RADIUS - 1);
    let y_boundaries = Point(RADIUS, f.h()  - RADIUS - 1);

    let delta_patch_threshold = 126u8;
    Vec::new()
}

/*
*/
