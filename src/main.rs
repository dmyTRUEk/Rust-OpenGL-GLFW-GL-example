// #![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;



pub mod random;
use crate::random::*;



const WINDOW_NAME: &str = "Hello Rust + OpenGL !";
const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 ourColor;

    void main () {
       // gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
       gl_Position = vec4(aPos, 1.0);
       ourColor = aColor;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    in vec3 ourColor;

    void main () {
       //FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
       FragColor = vec4(ourColor, 1.0f);
    }
"#;



unsafe fn compile_shader (shader_type: u32, shader_source: &str) -> u32 {
    let shader = gl::CreateShader(shader_type);
    let c_str_vert = CString::new(shader_source.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    // check for shader compile errors:
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1);      // subtract 1 to skip the trailing null character
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        let shader_type_name = match shader_type {
            gl::VERTEX_SHADER => { "VERTEX_SHADER" },
            gl::FRAGMENT_SHADER => { "FRAGMENT_SHADER" },
            _ => { "UNKNOWN_SHADER_TYPE" }
        };
        println!("ERROR::SHADER::{}::COMPILATION_FAILED\n{}", shader_type_name ,str::from_utf8(&info_log).unwrap());
    }

    shader
}



unsafe fn link_shader (vertex_shader: u32, fragment_shader: u32) -> u32 {
    // link shaders
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    // check for linking errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    shader_program
}



fn main () {
    // glfw: initialize and configure
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    let (mut window, events) = glfw.create_window(SCREEN_W, SCREEN_H, WINDOW_NAME, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut vertices: Vec<f32> = vec![
    //    X     Y    Z      R    G    B
        -0.5, -0.5, 0.0,   1.0, 0.0, 0.0, 
         0.0, -0.5, 0.0,   1.0, 0.0, 0.0, 
         0.0,  0.0, 0.0,   1.0, 0.0, 0.0, 

         0.5,  0.5, 0.0,   0.0, 1.0, 0.0, 
         0.0,  0.5, 0.0,   0.0, 1.0, 0.0, 
         0.0,  0.0, 0.0,   0.0, 1.0, 0.0, 

         0.0,  0.0, 0.0,   0.0, 0.0, 1.0,
         0.0,  0.0, 0.0,   0.0, 0.0, 1.0,
         0.0,  0.0, 0.0,   0.0, 0.0, 1.0,
    ];

    let (shader_program, vao) = unsafe {
        let vertex_shader   = compile_shader(gl::VERTEX_SHADER  , VERTEX_SHADER_SOURCE  );
        let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

        let shader_program = link_shader(vertex_shader, fragment_shader);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::DYNAMIC_DRAW
        );

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        // color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        
        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as
        // the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        // gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO,
        // but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs
        // (nor VBOs) when it's not directly necessary.
        // gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao)
    };

    let mut frames_drawed: u64 = 0;
    let mut should_add_new_triangle: bool = false;

    // main loop:
    while !window.should_close() {
        // events:
        process_events(&mut window, &events, &mut should_add_new_triangle);

        // render:
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            gl::UseProgram(shader_program);

            // println!("should_draw_third_triangle = {}", should_draw_third_triangle);

            if should_add_new_triangle {
                should_add_new_triangle = false;

                let new_vertices: Vec<f32> = vec![
                //    X     Y    Z      R    G    B
                    random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), 
                    random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), 
                    random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), 
                ];
                for item in &new_vertices {
                    vertices.push(*item);
                }
            }

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW
            );
            
            // seeing as we only have a single VAO there's no need to bind it every time,
            // but we'll do so to keep things a bit more organized
            gl::BindVertexArray(vao); 
            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32 / 2_i32);

            // glBindVertexArray(0); // no need to unbind it every time
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();

        frames_drawed += 1;
    }

    println!("Frames Drawed = {}", frames_drawed);
    println!("Finished OK!");
}



fn process_events (
        window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>,
        should_draw_third_triangle: &mut bool
        ) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            },
            glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                *should_draw_third_triangle = true;
            },
            _ => {}
        }
    }
}



