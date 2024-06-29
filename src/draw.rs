use crate::{FrameState, Vec2};

#[inline(always)]
pub fn line(
    frame: &mut FrameState,
    from: impl Into<Vec2>,
    to: impl Into<Vec2>,
    callback: impl Fn(&mut FrameState, Vec2),
) {
    let mut from = from.into();
    let to = to.into();

    let dx = (to.x - from.x).abs();
    let sx = if from.x < to.x { 1 } else { -1 };

    let dy = -(to.y - from.y).abs();
    let sy = if from.y < to.y { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        callback(frame, from);

        if from.x == to.x && from.y == to.y {
            break;
        }
        let err2 = 2 * err;
        if err2 >= dy {
            err += dy;
            from.x += sx;
        }
        if err2 <= dx {
            err += dx;
            from.y += sy;
        }
    }
}

pub fn circle(
    frame: &mut FrameState,
    center: impl Into<Vec2>,
    radius: u32,
    callback: impl Fn(&mut FrameState, Vec2),
) {
    let center = center.into();

    let mut err = -(radius as i32);
    let mut x = radius as i32;
    let mut y = 0;

    callback(frame, [center.x, center.y - radius as i32].into());

    while x >= y {
        callback(frame, [center.x + x, center.y + y].into());
        if x != y {
            callback(frame, [center.x + y, center.y + x].into());
        }

        if x != 0 {
            callback(frame, [center.x - x, center.y + y].into());
            if x != y {
                callback(frame, [center.x - y, center.y + x].into());
            }
        }

        if y != 0 {
            callback(frame, [center.x + x, center.y - y].into());
            if x != y {
                callback(frame, [center.x + y, center.y - x].into());
            }
        }

        if x != 0 && y != 0 {
            callback(frame, [center.x - x, center.y - y].into());
            if x != y {
                callback(frame, [center.x - y, center.y - x].into());
            }
        }

        err += y;
        y += 1;
        err += y;

        if err >= 0 {
            x -= 1;
            err -= 2 * x;
        }
    }
}
