use crate::grid::{GRID_HEIGHT, GRID_WIDTH};

pub(crate) fn bresenham_line(x0: usize, y0: usize, x1: usize, y1: usize) -> impl Iterator<Item = (usize, usize)> {
    let dx = (x1 as isize - x0 as isize).abs();
    let dy = (y1 as isize - y0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0 as isize;
    let mut y = y0 as isize;

    std::iter::from_fn(move || {
        if (x as usize == x1 && y as usize == y1) || x < 0 || y < 0 || x >= GRID_WIDTH as isize || y >= GRID_HEIGHT as isize {
            None
        } else {
            let current = (x as usize, y as usize);
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
            Some(current)
        }
    })
}