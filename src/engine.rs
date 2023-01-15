use glium::{Display, Program, Surface, uniform, VertexBuffer};
use glium::glutin::ContextBuilder;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::camera::Camera;
use crate::input::InputHandler;
use crate::shape::Shape;

pub struct Render {

}

impl Render {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new().with_title("voxel");
        let cb = ContextBuilder::new().with_depth_buffer(24);

        let fragment_shader = include_str!("shaders/fragment_shader.glsl");
        let vertex_shader = include_str!("shaders/vertex_shader.glsl");

        let shape = Shape::new_plane();
        let mut t = 0f32;

        let display = Display::new(
            wb,
            cb,
            &event_loop
        ).unwrap();

        display.get_framebuffer_dimensions();

        let program = Program::from_source(
            &display,
            vertex_shader,
            fragment_shader,
            None
        ).unwrap();

        let vertex_buf = VertexBuffer::new(
            &display,
            &shape
        ).unwrap();

        let indices = glium::index::NoIndices(
            glium::index::PrimitiveType::TrianglesList
        );

        event_loop.run(move |event, _, control_flow| {
            let mut target = display.draw();
            t += 0.0002;

            match event {
                Event::WindowEvent {
                    window_id: _,
                    event
                } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _
                    } => InputHandler::handle_key(input),
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        modifiers
                    } => InputHandler::handle_mouse(position),
                    _ => ()
                }
                _ => ()
            }
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            target.draw(
                &vertex_buf,
                &indices,
                &program,
                &uniform! {
                    r_matrix: Camera::r_matrix(t),
                    p_matrix: Camera::p_matrix(90.0, &display)
                },
                &Default::default()
            ).unwrap();

            target.finish().unwrap();
        });
    }
}
