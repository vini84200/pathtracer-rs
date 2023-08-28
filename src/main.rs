use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod color;
mod geometry;
mod raytracer;
mod renderer;
mod material;
mod camera;
mod object;
mod light;
mod world;



#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    env_logger::init();
    human_panic::setup_panic!();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Raytracer")
        .build(&event_loop)
        .unwrap();

    let mut state = renderer::state::State::new(window).await;
    state.init();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size);
            }
            WindowEvent::KeyboardInput{..} => {
                state.input(event);
            },
            WindowEvent::CursorMoved { ..} => {
                state.input(event);
            },
            WindowEvent::MouseInput { ..} => {
                state.input(event);
            },
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            state.window().request_redraw();
        }
        _ => {}
    });
}
