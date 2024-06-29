use pixels::wgpu::Color;
use pygmalion::run_window;
use pygmalion::window::event::VirtualKeyCode;
use pygmalion::window::event_loop::ControlFlow;

fn main() {
    run_window(|pixels, input, control_flow| {
        pixels.clear_color(Color::BLACK);

        if input.key_pressed(VirtualKeyCode::Escape) {
            *control_flow = ControlFlow::Exit;
        }

        let (mx, my) = pixels
            .window_pos_to_pixel(input.mouse().unwrap_or_default())
            .unwrap_or_default();

        let i = 4 * (my as u32 * pixels.texture().width() + mx as u32) as usize;

        let rgb = &mut pixels.frame_mut()[i..i + 4];
        rgb[0] = 0xFF;
        rgb[1] = 0xFF;
        rgb[2] = 0xFF;
        rgb[3] = 0xFF;
    });
}
