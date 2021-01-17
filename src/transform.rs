use glam;

pub struct Transform {
    pub scale: glam::Vec3,
    pub rotation: glam::Quat,
    pub position: glam::Vec3,
}

impl Transform {
    
    pub fn new() -> Transform {
        return Transform::default();
    }
    
    pub fn get_transform_matrix(&self) -> [[f32; 4]; 4] {
        return glam::Mat4::from_scale_rotation_translation(self.scale,self.rotation,self.position)
        .to_cols_array_2d();
    }

    pub fn set_position(&mut self, movement: glam::Vec3) {
        self.position = movement;
    }
    #[allow(dead_code)]
    fn to_std_coords(scale: &mut glam::Vec3, rotation: &mut glam::Quat, position: &mut glam::Vec3) {
        // se queda pero ya no hace falt ausarlo porque ahora voy a trabajar con
        // el sistema de coordenadas de blender, que es
        // z hacia arriba, y hacia adelante
        use glam::{Quat, Vec3};
        let mut oy: f32;
        let mut oz: f32;
        // Coordenadas respecto a blender.
        // Para pasar posiciones de blender a opengl, tenemos que
        // intercambiar los ejes y, z entre si ... y despues
        // invertir el signo del eje z de opengl (que seria el y en blender)
        // entonces opengl=blender tal que x=x ; y=z ; z=-y
        // posicion
        oy = Vec3::y(*position);
        oz = Vec3::z(*position);
        Vec3::set_y(position, oz);
        Vec3::set_z(position, -oy);
        // rotacion, hacemos lo mismo pero Quat no tiene funciones para acceder
        // a los campos asi que es manual nomas...
        *rotation = Quat::from_xyzw(
            Quat::x(*rotation),
            Quat::z(*rotation),
            -Quat::y(*rotation),
            Quat::w(*rotation),
        );
        // escala, igual que para la posicion pero sin invertir el signo
        // del eje z porque sino invierte las normales tambien
        oy = Vec3::y(*scale);
        oz = Vec3::z(*scale);
        Vec3::set_y(scale, oz);
        Vec3::set_z(scale, oy);
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            scale: glam::Vec3::new(1.0, 1.0, 1.0),
            rotation: glam::quat(0.0,0.0,0.0,1.0),
            position: glam::Vec3::new(0.0, 0.0, 0.0)
        }
    }
}
