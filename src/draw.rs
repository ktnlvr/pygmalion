use crate::Vec2;

#[inline(always)]
pub fn line(pixels: &mut pixels::Pixels, from: impl Into<Vec2>, to: impl Into<Vec2>, callback: impl Fn(&mut pixels::Pixels, Vec2)) {
    let from = from.into();
    let to = to.into();

    let dx = to.x - from.x;
    let dy = to.y - from.y;

    let mut d = 2 * dy - dx;
    let mut y = from.y;

    for x in from.x..to.x {
        if x as u32 > pixels.texture().size().width || y as u32 > pixels.texture().size().height {
            return
        }

        callback(pixels, Vec2::from([x, y]));

        if d > 0 { 
            y += 1;

            d -= 2 * dx;
        }
        d += 2 * dy;

    }
}
