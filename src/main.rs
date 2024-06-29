use pygmalion::run_window;
use pygmalion::window::event::VirtualKeyCode;
use pygmalion::window::event_loop::ControlFlow;

fn main() {
    run_window(|input, control_flow| {
        if input.key_pressed(VirtualKeyCode::Escape) {
            *control_flow = ControlFlow::Exit;
        }
    });
}
