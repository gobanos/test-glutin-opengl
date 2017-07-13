#![recursion_limit = "1024"]

extern crate gl;
extern crate glutin;
extern crate libc;
extern crate glm;
extern crate num_traits;
#[macro_use] extern crate error_chain;

mod errors {
    error_chain!{}
}

use glutin::GlContext;

use errors::*;
use gl::types::*;
use num_traits::identities::One;

use std::ffi::CString;
use std::mem;

static VERTEX_BUFFER_DATA: &[GLfloat] = &[
    // triangle 1
    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,

    // triangle 2
     1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,

    // triangle 3
     1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,

    // triangle 4
     1.0,  1.0, -1.0,
     1.0, -1.0, -1.0,
    -1.0, -1.0, -1.0,

    // triangle 5
    -1.0, -1.0, -1.0,
    -1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,

    // triangle 6
     1.0, -1.0,  1.0,
    -1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,

    // triangle 7
    -1.0,  1.0,  1.0,
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,

    // triangle 8
     1.0,  1.0,  1.0,
     1.0, -1.0, -1.0,
     1.0,  1.0, -1.0,

    // triangle 9
     1.0, -1.0, -1.0,
     1.0,  1.0,  1.0,
     1.0, -1.0,  1.0,

    // triangle 10
     1.0,  1.0,  1.0,
     1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,

    // triangle 11
     1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,

    // triangle 12
     1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
     1.0, -1.0,  1.0,
];

static COLOR_BUFFER_DATA: &[GLfloat] = &[
    0.583, 0.771, 0.014,
    0.609, 0.115, 0.436,
    0.327, 0.483, 0.844,
    0.822, 0.569, 0.201,
    0.435, 0.602, 0.223,
    0.310, 0.747, 0.185,
    0.597, 0.770, 0.761,
    0.559, 0.436, 0.730,
    0.359, 0.583, 0.152,
    0.483, 0.596, 0.789,
    0.559, 0.861, 0.639,
    0.195, 0.548, 0.859,
    0.014, 0.184, 0.576,
    0.771, 0.328, 0.970,
    0.406, 0.615, 0.116,
    0.676, 0.977, 0.133,
    0.971, 0.572, 0.833,
    0.140, 0.616, 0.489,
    0.997, 0.513, 0.064,
    0.945, 0.719, 0.592,
    0.543, 0.021, 0.978,
    0.279, 0.317, 0.505,
    0.167, 0.620, 0.077,
    0.347, 0.857, 0.137,
    0.055, 0.953, 0.042,
    0.714, 0.505, 0.345,
    0.783, 0.290, 0.734,
    0.722, 0.645, 0.174,
    0.302, 0.455, 0.848,
    0.225, 0.587, 0.040,
    0.517, 0.713, 0.338,
    0.053, 0.959, 0.120,
    0.393, 0.621, 0.362,
    0.673, 0.211, 0.457,
    0.820, 0.883, 0.371,
    0.982, 0.099, 0.879,
];

fn run() -> Result<()> {
    let mut event_loop = glutin::EventsLoop::new();
    let window = build_window(&event_loop)?;

    // Create VAO
    let mut vertex_array_id = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }

    // Create Vertex Buffer
    let mut vertex_buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(VERTEX_BUFFER_DATA) as GLsizeiptr,
            VERTEX_BUFFER_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW
        );
    }

    // Create Color Buffer
    let mut color_buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut color_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(COLOR_BUFFER_DATA) as GLsizeiptr,
            COLOR_BUFFER_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW
        );
    }

    let program = load_shaders(
        "resources/shaders/simple-vertex.glsl",
        "resources/shaders/simple-fragment.glsl"
    ).chain_err(|| "Failed to load shaders")?;

    unsafe { gl::UseProgram(program) };

    let (width, height) = window.get_inner_size_pixels().unwrap_or((1024, 768));

    let projection = glm::ext::perspective(
        glm::radians(45.0),
        width as f32 / height as f32,
        0.1,
        100.0
    );

    let view = glm::ext::look_at(
        glm::vec3(4.0, 3.0, 3.0),   // position
        glm::vec3(0.0, 0.0, 0.0),   // look at
        glm::vec3(0.0, 1.0, 0.0),   // up vector
    );

    let model = glm::Matrix4::one();

    let mvp = projection * view * model;

    unsafe {
        let name = CString::new("MVP")
            .chain_err(|| "Failed to parse MVP")?;
        let matrix_id = gl::GetUniformLocation(program, name.as_ptr());
        gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, mem::transmute(mvp.as_array()));
    }

    let mut running = true;
    while running {
        event_loop.poll_events(|event| {
            use glutin::{ Event, WindowEvent, KeyboardInput, VirtualKeyCode };

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => running = false,
                    WindowEvent::Resized(w, h) => window.resize(w, h),
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape), ..
                        } => running = false,
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        unsafe {
            // Bind vertex buffer
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::VertexAttribPointer(
                0,                  // attribute 0. No particular reason for 0, but must match the layout in the shader.
                3,                  // size
                gl::FLOAT,          // type
                gl::FALSE,          // normalized?
                0,                  // stride
                0 as *const _       // array buffer offset
            );

            // Bind color buffer
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
            gl::VertexAttribPointer(
                1,                  // attribute 1. No particular reason for 1, but must match the layout in the shader.
                3,                  // size
                gl::FLOAT,          // type
                gl::FALSE,          // normalized?
                0,                  // stride
                0 as *const _       // array buffer offset
            );

            // Starting from vertex 0, 3 vertices total -> 1 triangle
            gl::DrawArrays(gl::TRIANGLES, 0, VERTEX_BUFFER_DATA.len() as i32);
            gl::DisableVertexAttribArray(0);
        }

        window.swap_buffers()
            .chain_err(|| "Unable to swap buffers")?;
    }

    Ok(())
}

fn load_shaders(vertex_file: &str, fragment_file: &str) -> Result<GLuint> {
    use std::fs::File;
    use std::io::Read;

    use std::ptr::{ null, null_mut };

    let mut vertex_file = File::open(vertex_file)
        .chain_err(|| "Unable to open vertex shader file")?;
    let mut fragment_file = File::open(fragment_file)
        .chain_err(|| "Unable to open fragment shader file")?;

    let vertex_shader_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let fragment_shader_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

    let mut vertex_shader_code = String::new();
    vertex_file.read_to_string(&mut vertex_shader_code)
        .chain_err(|| "Unable to read vertex shader file")?;

    let mut fragment_shader_code = String::new();
    fragment_file.read_to_string(&mut fragment_shader_code)
        .chain_err(|| "Unable to read fragment shader file")?;

    let mut result = 0;
    let mut info_log_length = 0;

    // Compile vertex shader
    let c_vertex_shader_code = CString::new(vertex_shader_code)
        .chain_err(|| "Failed to convert vertex shader code to c string")?;
    unsafe {
        gl::ShaderSource(vertex_shader_id, 1, &c_vertex_shader_code.as_ptr(), null());
        gl::CompileShader(vertex_shader_id);
    }

    //  Check vertex shader
    unsafe {
        gl::GetShaderiv(vertex_shader_id, gl::COMPILE_STATUS, &mut result);
        gl::GetShaderiv(vertex_shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
    }

    if info_log_length > 0 {
        let mut error_message = Vec::with_capacity(info_log_length as usize + 1);
        let error_ptr = error_message.as_mut_ptr();
        unsafe {
            gl::GetShaderInfoLog(
                vertex_shader_id,
                info_log_length,
                null_mut(),
                error_ptr
            );
        }
        let error_message = unsafe { CString::from_raw(error_ptr) }
            .into_string()
            .chain_err(|| "Failed to parse error message !")?;

        bail!(error_message);
    }


    // Compile fragment shader
    let c_fragment_shader_code = CString::new(fragment_shader_code)
        .chain_err(|| "Failed to convert fragment shader code to c string")?;
    unsafe {
        gl::ShaderSource(fragment_shader_id, 1, &c_fragment_shader_code.as_ptr(), null());
        gl::CompileShader(fragment_shader_id);
    }

    // Check fragment shader
    unsafe {
        gl::GetShaderiv(fragment_shader_id, gl::COMPILE_STATUS, &mut result);
        gl::GetShaderiv(fragment_shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
    }

    if info_log_length > 0 {
        let mut error_message = Vec::with_capacity(info_log_length as usize + 1);
        let error_ptr = error_message.as_mut_ptr();
        unsafe {
            gl::GetShaderInfoLog(
                fragment_shader_id,
                info_log_length,
                null_mut(),
                error_ptr
            );
        }
        let error_message = unsafe { CString::from_raw(error_ptr) }
            .into_string()
            .chain_err(|| "Failed to parse error message !")?;

        bail!(error_message);
    }

    // Link the program
    let program_id = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program_id, vertex_shader_id);
        gl::AttachShader(program_id, fragment_shader_id);
        gl::LinkProgram(program_id);
    }

    // Check the program
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut result);
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
    }

    if info_log_length > 0 {
        let mut error_message = Vec::with_capacity(info_log_length as usize + 1);
        let error_ptr = error_message.as_mut_ptr();
        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                info_log_length,
                null_mut(),
                error_ptr
            );
        }
        let error_message = unsafe { CString::from_raw(error_ptr) }
            .into_string()
            .chain_err(|| "Failed to parse error message !")?;

        bail!(error_message);
    }

    unsafe {
        gl::DetachShader(program_id, vertex_shader_id);
        gl::DetachShader(program_id, fragment_shader_id);

        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
    }

    Ok(program_id)
}

fn build_window(event_loop: &glutin::EventsLoop) -> Result<glutin::GlWindow> {
    let window = glutin::WindowBuilder::new()
        .with_title("Hello Glutin !")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let gl_window = glutin::GlWindow::new(window, context, &event_loop)
        .chain_err(|| "Unable to build window")?;

    unsafe { gl_window.make_current() }
        .chain_err(|| "Unable to make window current")?;

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.4, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    Ok(gl_window)
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}