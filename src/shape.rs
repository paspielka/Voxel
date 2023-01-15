use glium::implement_vertex;

pub struct Shape {

}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3]
}
implement_vertex!(Vertex, position);

impl Shape {
    pub fn new_cube() -> Vec<Vertex> {

        let vertices: [Vertex; 36] = [
            // front
            Vertex { position: [ 0.5,-0.5, 0.0] },
            Vertex { position: [-0.5,-0.5, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0] },

            Vertex { position: [ 0.5,-0.5, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0] },
            Vertex { position: [ 0.5, 0.5, 0.0] },

            // back
            Vertex { position: [ 0.5,-0.5,-1.0] },
            Vertex { position: [-0.5,-0.5,-1.0] },
            Vertex { position: [-0.5, 0.5,-1.0] },

            Vertex { position: [ 0.5,-0.5,-1.0] },
            Vertex { position: [-0.5, 0.5,-1.0] },
            Vertex { position: [ 0.5, 0.5,-1.0] },

            // right
            Vertex { position: [ 0.5,-0.5,-1.0] },
            Vertex { position: [ 0.5,-0.5, 0.0] },
            Vertex { position: [ 0.5, 0.5, 0.0] },

            Vertex { position: [ 0.5,-0.5,-1.0] },
            Vertex { position: [ 0.5, 0.5, 0.0] },
            Vertex { position: [ 0.5, 0.5,-1.0] },

            // left
            Vertex { position: [-0.5,-0.5,-1.0] },
            Vertex { position: [-0.5,-0.5, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0] },

            Vertex { position: [-0.5,-0.5,-1.0] },
            Vertex { position: [-0.5, 0.5, 0.0] },
            Vertex { position: [-0.5, 0.5,-1.0] },

            // bottom
            Vertex { position: [ 0.5,-0.5, 0.0] },
            Vertex { position: [-0.5,-0.5, 0.0] },
            Vertex { position: [-0.5,-0.5,-1.0] },

            Vertex { position: [ 0.5,-0.5, 0.0] },
            Vertex { position: [-0.5,-0.5,-1.0] },
            Vertex { position: [ 0.5,-0.5,-1.0] },

            // bottom
            Vertex { position: [ 0.5, 0.5, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0] },
            Vertex { position: [-0.5, 0.5,-1.0] },

            Vertex { position: [ 0.5, 0.5, 0.0] },
            Vertex { position: [-0.5, 0.5,-1.0] },
            Vertex { position: [ 0.5, 0.5,-1.0] },
        ];
        vertices.to_vec()
    }

    pub fn new_terrain() {
        for y in 0..10 {
            for y in 0..10 {

            }
        }
    }

    pub fn add_pos(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        let mut new_pos = [0f32; 3];

        for x in 0..a.len() {
            new_pos[x] = a[x] + b[x];
        }
        new_pos
    }
}
