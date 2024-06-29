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
