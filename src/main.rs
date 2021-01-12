#[macro_use]
extern crate glium;
extern crate image;

pub mod mesh;

use std::fs;
use glam;
use mesh::Mesh;

fn main() {

    // comienza la carga de modelo
    // "resources/meshes/abstract.obj"

    let sample_mesh: Mesh = Mesh::insta_load("resources/meshes/abstract.obj");
    let lista_indices = sample_mesh.index_list;
    let lista_vertices = sample_mesh.vertex_list;

    // terminada la carga

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    // 1. The **winit::EventsLoop** for handling events.
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(500.0, 500.0))
        .with_title("Look mama Im opengl-ing in rust!!");

    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &lista_vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,&lista_indices).unwrap();

    let vertex_shader_src = fs::read_to_string("resources/shaders/vert-shader.glsl")
        .expect("\n### No se encontro el archivo vertex shader. \n");
    let fragment_shader_src = fs::read_to_string("resources/shaders/frag-shader.glsl")
        .expect("\n### No se encontro el archivo fragment shader. \n");

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    let mut delta_time: f32 = -7.0;
    let light = [-1.0, 0.4, 0.9f32];

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };
    
    let mut scale = glam::vec3(1.0, 1.0, 1.0);
    let mut rotation = glam::quat(0.0, 0.0, 0.0, 1.0);
    let mut position = glam::vec3(0.0,0.0,0.0);
    std_coords(&mut scale, &mut rotation, &mut position);
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        delta_time += 0.035;

        if delta_time > 5.0 {
            delta_time = -7.0;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);


        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.05, 0.0, 1.0), 1.0);




        let transform_matrix = glam::Mat4::from_scale_rotation_translation(scale, rotation, position).to_cols_array_2d();
        let perspective_matrix = generate_perspective_matrix(target.get_dimensions(), 30.0);
        let view_matrix = generate_view_matrix(&[7.0, 5.0, 7.0], &[-7.0, -5.0, -7.0], &[0.0, 1.0, 0.0]);
        
        let uniforms = uniform! {
            t_matrix: transform_matrix,
            p_matrix: perspective_matrix,
            v_matrix: view_matrix,
            u_light: light,
        };

        target.draw(
                    &vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniforms,
                    &params,
            ).unwrap();

        //target.finish().unwrap();

        ////////// proximo draw call
        
        let mut scale = glam::vec3(1.0, 1.0, 1.0);
        let mut rotation = glam::quat(-0.4, -0.1, 0.1, 1.0);
        let mut position = glam::vec3(-0.7,-1.0,delta_time);
        std_coords(&mut scale, &mut rotation, &mut position);
        
        let transform_matrix = glam::Mat4::from_scale_rotation_translation(scale, rotation, position).to_cols_array_2d();
        let perspective_matrix = generate_perspective_matrix(target.get_dimensions(), 30.0);
        let view_matrix = generate_view_matrix(&[7.0, 5.0, 7.0], &[-7.0, -5.0, -7.0], &[0.0, 1.0, 0.0]);

        let uniforms = uniform! {
            t_matrix: transform_matrix,
            p_matrix: perspective_matrix,
            v_matrix: view_matrix,
            u_light: light,
        };

        target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniforms,
            &params,
        ).unwrap();

        // fin del 2do draw call


        target.finish().unwrap();


    });
}

fn generate_perspective_matrix(dimensions: (u32, u32), deg_fov: f32) -> [[f32; 4]; 4] {
    let fov: f32 = deg_fov * 3.141592 / 180.0;
    let aspect_ratio = dimensions.0 as f32 / dimensions.1 as f32;
    let result = glam::Mat4::perspective_rh(fov, aspect_ratio, 0.1, 1024.0).to_cols_array_2d();
    return result;
}

fn generate_view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    
    use glam::{Vec3};

    let vposition = glam::vec3(position[0], position[1], position[2]);
    let vdirection = glam::vec3(direction[0], direction[1], direction[2]);
    let vup = glam::vec3(up[0], up[1], up[2]);

    let zaxis = Vec3::normalize(vposition - vdirection);
    let xaxis = Vec3::normalize(Vec3::cross(vup,zaxis));
    let yaxis = Vec3::cross(zaxis,xaxis);

    [
        [Vec3::x(xaxis)               , Vec3::x(yaxis)               , Vec3::x(zaxis), 0.0],
        [Vec3::y(xaxis)               , Vec3::y(yaxis)               , Vec3::y(zaxis), 0.0],
        [Vec3::z(xaxis)               , Vec3::z(yaxis)               , Vec3::z(zaxis), 0.0],
        [-(Vec3::dot(xaxis,vposition)), -(Vec3::dot(yaxis,vposition)), -(Vec3::dot(zaxis,vposition)), 1.0],
    ]
}

fn std_coords (scale: &mut glam::Vec3, rotation: &mut glam::Quat, position: &mut glam::Vec3) {
    
    use glam::{Vec3,Quat};
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
    *rotation = Quat::from_xyzw(Quat::x(*rotation), 
                                Quat::z(*rotation), 
                                -Quat::y(*rotation), 
                                Quat::w(*rotation));

    // escala, igual que para la posicion pero sin invertir el signo
    // del eje z porque sino invierte las normales tambien
    oy = Vec3::y(*scale);
    oz = Vec3::z(*scale);
    Vec3::set_y(scale, oz);
    Vec3::set_z(scale, oy);

}


