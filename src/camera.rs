pub(super) struct Camera {
    position: glam::Vec3,
    target: glam::Vec3,
    dimensions: (u32,u32),
    deg_fov: f32,
    roll: f32
}

#[allow(dead_code)]
impl Camera {
    pub fn new(new_fov: f32, new_dimensions: (u32,u32)) -> Camera{
        Camera {
            position: glam::Vec3::zero(),
            target: glam::Vec3::zero(),
            dimensions: new_dimensions,
            deg_fov: new_fov,
            roll: 0.0
        }
    }

    pub fn set_position(&mut self, new_position: glam::Vec3) {
        self.position = new_position;
    }

    pub fn set_target(&mut self, new_target: glam::Vec3)  {
        self.target = new_target;
    }

    pub fn set_dimensions(&mut self,nd: (u32,u32)) {
        self.dimensions = nd;
    }

    pub fn set_roll(&mut self, new_roll: f32) {
        self.roll = new_roll;
    }

    pub fn set_roll_deg(&mut self, new_roll: f32) {
        self.roll = new_roll * 3.141592 / 180.0;
    }

    pub fn make_view_matrix(&self) -> [[f32; 4]; 4] {
        // x por el seno , z por el coseno
        let up_vector = glam::vec3(self.roll.sin(), 0.0, self.roll.cos());
        let result = glam::Mat4::look_at_rh(self.position, self.target-self.position, up_vector).to_cols_array_2d();
        return result;
    }

    pub fn make_perspective_matrix(&self) -> [[f32; 4]; 4] {
        let fov: f32 = self.deg_fov * 3.141592 / 180.0;
        let aspect_ratio = self.dimensions.0 as f32 / self.dimensions.1 as f32;
        let result = glam::Mat4::perspective_rh(fov, aspect_ratio, 0.1, 1024.0).to_cols_array_2d();
        return result;
    }
}

pub(super) struct Viewport {
    w: f32,
    h: f32,
}

impl Viewport {
    pub fn new(nw: f32, nh: f32) -> Viewport {
        Viewport {
            w: nw,
            h: nh
        }
    }

    pub fn w(&self) -> f32 {
        let rw = self.w;
        return rw;
    }

    pub fn h(&self) -> f32 {
        let rh = self.h;
        return rh;
    }

    pub fn uw(&self) -> u32 {
        let ruw = self.w as u32;
        return ruw;
    }

    pub fn uh(&self) -> u32 {
        let ruh = self.h as u32;
        return ruh;
    }

    pub fn ud(&self) -> (u32,u32) {
        let rud = (self.uw(),self.uh());
        return rud;
    }
}