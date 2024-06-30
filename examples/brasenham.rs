use pygmalion::{
    circle, line, run_window,
    window::{event::VirtualKeyCode, event_loop::ControlFlow},
    FrameState,
};

fn main() {
    run_window(Default::default(), |mut frame| {
        frame.clear();

        if frame.input.key_pressed(VirtualKeyCode::Escape) {
            *frame.control_flow = ControlFlow::Exit;
        }

        let mouse_pos = frame.mouse_position();

        let yellow = |frame: &mut FrameState, p| {
            if !frame.is_inside(p) {
                return;
            }

            frame.put(p, [0xFF, 0xFF, 0x00]);
        };

        line(&mut frame, [0, 0], mouse_pos, yellow);
        circle(
            &mut frame,
            mouse_pos,
            mouse_pos
                .map(|n| n.abs())
                .iter()
                .cloned()
                .max()
                .unwrap_or_default() as u32
                / 2,
            yellow,
        )
    });
}
