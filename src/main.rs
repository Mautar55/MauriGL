#[macro_use]
extern crate glium;
extern crate image;
use std::fs;
use std::io::BufReader;
use obj::{load_obj, Obj};

fn main() {

    // comienza la carga de modelo

    let input = BufReader::new(fs::File::open("resources/meshes/abstract.obj").expect("### No se encontro el archivo."));
    let obj: Obj = load_obj(input).expect("### No se pudo cargar el objeto.");

    let lista_vertices = obj.vertices;
    let mut pre_lista_indices = obj.indices;
    let mut lista_indices = Vec::<u16>::new();
    // arreglando la lista! Porque los indices tienen las caras alreves!
    
    let longitud = pre_lista_indices.len()%3;
    assert_eq!(longitud,0);
    let mut iterador_indices = pre_lista_indices.iter_mut();
    
    loop {
        let next1 = iterador_indices.next();
        if next1.is_none() {
            break;
        }
        let next1 = next1.unwrap();
        let next2 = iterador_indices.next().unwrap();
        let next3 = iterador_indices.next().unwrap();
        
        println!(">>>   Lista de indices {} {} {}",next1,next2,next3);

        lista_indices.push(*next3);
        lista_indices.push(*next2);
        lista_indices.push(*next1);

    }
    drop(pre_lista_indices);

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

    let mut delta_time: f32 = -0.5;
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

        delta_time += 0.01;

        if delta_time > 0.5 {
            delta_time = -0.5;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.2, 0.0, 1.0), 1.0);

        let transform_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let perspective_matrix = generate_perspective_matrix(target.get_dimensions(), 30.0);

        let view_matrix =
            generate_view_matrix(&[7.0, 5.0, 7.0], &[-7.0, -5.0, -7.0], &[0.0, 1.0, 0.0]);

        let uniforms = uniform! {
            t_matrix: transform_matrix,
            p_matrix: perspective_matrix,
            v_matrix: view_matrix,
            u_light: light,
        };

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &params,
            )
            .unwrap();

        target.finish().unwrap();
    });
}

fn generate_perspective_matrix(dimensions: (u32, u32), deg_fov: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = dimensions.0 as f32 / dimensions.1 as f32;

    let fov: f32 = deg_fov * 3.141592 / 180.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}

fn generate_view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    
    // Coordenadas respecto a blender.
    // Para pasar posiciones de blender a opengl, tenemos que invertir
    // los ejes y, z entre si y despues invertir el signo
    // de los ejes x, z de opengl (x, y en blender)
    
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
