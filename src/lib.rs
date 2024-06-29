mod draw;

pub use pixels;

pub mod window {
    pub use winit::*;
    pub use winit_input_helper::*;
}

pub use draw::*;

mod misc {
    use pixels::{PixelsBuilder, SurfaceTexture};
    use winit::{
        dpi::LogicalSize,
        event::Event,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use winit_input_helper::WinitInputHelper;

    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 240;
    
    pub fn run_window(event_callback: impl Fn(&mut WinitInputHelper, &mut ControlFlow) + 'static) {
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

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            PixelsBuilder::new(16 * 4, 9 * 4, surface_texture)
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

            event_callback(&mut input, control_flow);
        });
    }
}

pub use misc::run_window;
