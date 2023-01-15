use glium::implement_vertex;

pub struct Shape {

}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3]
}
implement_vertex!(Vertex, position);

impl Shape {
    pub fn new_plane() -> Vec<Vertex> {
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
}
