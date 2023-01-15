use glam::{Mat4, Vec3};
use glium::Display;

pub struct Camera {

}

impl Camera {
    pub fn r_matrix(t: f32) -> [[f32; 4]; 4] {
        // rotation matrix
        [
            [ t.cos(), 0.0, t.sin(), 0.0],
            [ 0.0,     1.0, 0.0,     0.0],
            [-t.sin(), 0.0, t.cos(), 0.0],
            [ 0.0,     0.0, 0.0,     1.0]
        ]
    }

    pub fn p_matrix(fov: f32, display: &Display) -> [[f32; 4]; 4] {
        // perspective matrix
        let (width, height) = display.get_framebuffer_dimensions();

        let projection_matrix = Mat4::perspective_rh_gl(
            fov.to_radians(),
            (width as f32)/(height as f32),
            0.1,
            1000.0
        );

        let view_matrix = Mat4::from_translation(
            Vec3::new(0.0, 0.0, -2.0)
        );

        let matrix = projection_matrix * view_matrix;
        matrix.to_cols_array_2d()
    }
}