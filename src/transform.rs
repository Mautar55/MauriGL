use glam;

pub struct Transform {
    scale: glam::Vec3,
    rotation: glam::Quat,
    position: glam::Vec3,
}

impl Transform {

    fn to_std_coords(scale: &mut glam::Vec3, rotation: &mut glam::Quat, position: &mut glam::Vec3) {
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
