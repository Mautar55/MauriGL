extern crate glium;
use super::transform::Transform;

pub(super) struct Camera {
    pub transform: Transform,
    pub dimensions: (u32,u32),
    pub deg_fov: f32
}

impl Camera {
    pub fn new(new_fov: f32, new_transform: Transform) -> Camera{
        Camera {
            transform: new_transform,
            dimensions: (0,0),
            deg_fov: new_fov
        }
    }

    pub fn set_dimensions(&mut self,nd: (u32,u32)) {
        self.dimensions = nd;
    }

    pub fn make_view_matrix(&self) -> [[f32; 4]; 4] {
        let pos = self.transform.position;
        let e_rot = self.transform.rotation.to_axis_angle();
        let result = glam::Mat4::look_at_rh(pos, -pos, glam::Vec3::unit_z()).to_cols_array_2d();
        return result;
    }

    pub fn make_perspective_matrix(&self) -> [[f32; 4]; 4] {;
        let fov: f32 = self.deg_fov * 3.141592 / 180.0;
        let aspect_ratio = self.dimensions.0 as f32 / self.dimensions.1 as f32;
        let result = glam::Mat4::perspective_rh(fov, aspect_ratio, 0.1, 1024.0).to_cols_array_2d();
        return result;
    }
}