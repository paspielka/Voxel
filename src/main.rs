use crate::engine::Render;

mod engine;
mod camera;
mod shape;
mod input;

fn main() {
    let mut render = Render::new();
    render.run();
}
