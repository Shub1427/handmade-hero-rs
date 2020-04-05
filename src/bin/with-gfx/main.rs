use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn run(window: Window, event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event, window_id, ..
            } => match event {
                WindowEvent::CloseRequested if window_id == window.id() => {
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } => {
                    println!(">>>>>>>>>>>>>>>> Window will Redraw <<<<<<<<<<<<<<<<");
                    window.request_redraw();
                }
                WindowEvent::Resized { .. } => {
                    println!(">>>>>>>>>>>>>>> Resized <<<<<<<<<<<<<<");
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) => {}
            _ => (),
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(1200.0, 500.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    run(window, event_loop);
}
