use pixels::wgpu::Color;
use pygmalion::window::event::VirtualKeyCode;
use pygmalion::window::event_loop::ControlFlow;
use pygmalion::{run_window, FrameState};

fn main() {
    run_window(
        |FrameState {
             pixels,
             input,
             control_flow,
             dimensions,
         }| {
            pixels.clear_color(Color::BLACK);

            if input.key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
            }

            let (mx, my) = pixels
                .window_pos_to_pixel(input.mouse().unwrap_or_default())
                .unwrap_or_default();

            let i = 4 * (my as u32 * dimensions.y + mx as u32) as usize;

            let rgb = &mut pixels.frame_mut()[i..i + 4];
            rgb[0] = 0xFF;
            rgb[1] = 0xFF;
            rgb[2] = 0xFF;
            rgb[3] = 0xFF;
        },
    );
}
