use itertools::{iproduct, Itertools};
use glium::implement_vertex;
use rand::Rng;

pub struct Shape {

}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3]
}
implement_vertex!(Vertex, position);

impl Shape {
    pub fn new_cube(pos: [f32; 3]) -> Vec<Vertex> {

        let vertices: [Vertex; 36] = [
            // front
            Vertex { position: Self::add_pos([ 0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5, 0.0], pos) },

            Vertex { position: Self::add_pos([ 0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([ 0.5, 0.5, 0.0], pos) },

            // back
            Vertex { position: Self::add_pos([ 0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([-0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5,-1.0], pos) },

            Vertex { position: Self::add_pos([ 0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([ 0.5, 0.5,-1.0], pos) },

            // right
            Vertex { position: Self::add_pos([ 0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([ 0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([ 0.5, 0.5, 0.0], pos) },

            Vertex { position: Self::add_pos([ 0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([ 0.5, 0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([ 0.5, 0.5,-1.0], pos) },

            // left
            Vertex { position: Self::add_pos([-0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([-0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5, 0.0], pos) },

            Vertex { position: Self::add_pos([-0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5,-1.0], pos) },

            // bottom
            Vertex { position: Self::add_pos([ 0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5,-0.5,-1.0], pos) },

            Vertex { position: Self::add_pos([ 0.5,-0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5,-0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([ 0.5,-0.5,-1.0], pos) },

            // bottom
            Vertex { position: Self::add_pos([ 0.5, 0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5,-1.0], pos) },

            Vertex { position: Self::add_pos([ 0.5, 0.5, 0.0], pos) },
            Vertex { position: Self::add_pos([-0.5, 0.5,-1.0], pos) },
            Vertex { position: Self::add_pos([ 0.5, 0.5,-1.0], pos) },
        ];
        vertices.to_vec()
    }

    pub fn new_shape() -> Vec<Vertex> {
        let mut shape: Vec<Vertex> = vec![];

        for (x, y, z) in iproduct!(0..=10, 0..=10, 0..=10) {
            let mut rng = rand::thread_rng();
            let mut num = rng.gen_range(0..=3);

            if num != 3 {
                continue;
            }

            for v in Self::new_cube([x as f32, y as f32, z as f32]) {
                shape.push(v);
            }
        }
        shape
    }

    pub fn add_pos(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        let mut new_pos = [0f32; 3];

        for x in 0..a.len() {
            new_pos[x] = a[x] + b[x];
        }
        new_pos
    }
}
