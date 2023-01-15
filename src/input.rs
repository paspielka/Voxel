use winit::dpi::PhysicalPosition;
use winit::event::KeyboardInput;

pub struct InputHandler {

}

impl InputHandler {
    pub fn handle_key(input: KeyboardInput) {
        println!("{:?}", input.virtual_keycode);
    }

    pub fn handle_mouse(position: PhysicalPosition<f64>) {
        println!("{:?}", position)
    }
}