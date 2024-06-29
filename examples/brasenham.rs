use pygmalion::{
    line, run_window,
    window::{event::VirtualKeyCode, event_loop::ControlFlow},
};

fn main() {
    run_window(|mut frame| {
        frame.clear();

        if frame.input.key_pressed(VirtualKeyCode::Escape) {
            *frame.control_flow = ControlFlow::Exit;
        }

        let mouse_pos = frame.mouse_position();

        line(&mut frame, [0, 0], mouse_pos, |frame, p| {
            let color = frame.get_mut(p);
            color[0] = 0xFF;
            color[1] = 0xFF;
            color[2] = 0x00;
            color[3] = 0xFF;
        });
    });
}
