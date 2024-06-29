mod draw;

pub type Vec2 = Vector2<i32>;

use mint::Vector2;
pub use pixels;

pub mod window {
    pub use winit::*;
    pub use winit_input_helper::*;
}

pub use draw::*;

pub struct FrameState<'pixels, 'input, 'control> {
    pub pixels: &'pixels mut Pixels,
    pub input: &'input mut WinitInputHelper,
    pub control_flow: &'control mut ControlFlow,

    pub dimensions: mint::Vector2<u32>,
}

mod misc {
    use mint::Vector2;
    use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
    use winit::{
        dpi::LogicalSize,
        event::Event,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use winit_input_helper::WinitInputHelper;

    use crate::FrameState;

    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 240;
    
    pub fn run_window(event_callback: impl Fn(FrameState<'_, '_, '_>) + 'static) {
        // TODO: don't be silent about errors, that's unhealthy

        env_logger::init();
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let scalar = 4;
        let width = 16 * scalar;
        let height = 16 * scalar;

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            PixelsBuilder::new(width, height, surface_texture)
                .build()
                .unwrap()
        };

        event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            if input.update(&event) {
                if input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Some(size) = input.window_resized() {
                    if pixels.resize_surface(size.width, size.height).is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                window.request_redraw();
            }

            let state = FrameState {
                pixels: &mut pixels,
                input: &mut input,
                control_flow,
                dimensions: Vector2::from([width, height]),
            };

            event_callback(state);
        });
    }
}

pub use misc::run_window;
use pixels::Pixels;
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;
