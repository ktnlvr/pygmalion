use pygmalion::{
    circle, line, run_window,
    window::{event::VirtualKeyCode, event_loop::ControlFlow},
    FrameState,
};

fn main() {
    run_window(|mut frame| {
        frame.clear();

        if frame.input.key_pressed(VirtualKeyCode::Escape) {
            *frame.control_flow = ControlFlow::Exit;
        }

        let mouse_pos = frame.mouse_position();

        let yellow = |frame: &mut FrameState, p| {
            if !frame.is_inside(p) {
                return;
            }

            let color = frame.get_mut(p);
            color[0] = 0xFF;
            color[1] = 0xFF;
            color[2] = 0x00;
            color[3] = 0xFF;
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
                .unwrap_or_default() as u32 / 2,
            yellow,
        )
    });
}
