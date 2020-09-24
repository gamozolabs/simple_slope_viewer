use std::ffi::CString;
use std::time::{Instant, Duration};

use gl::types::*;

use sdl2::event::Event;
use sdl2::video::SwapInterval;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

// Vertex data
static VERTEX_DATA: [GLfloat; 9] =
    [0.0, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];

// Shader sources
static VS_SRC: &'static str = "
#version 150
in vec3 position;
uniform vec3 origin;
uniform vec3 target;

mat3 calcLookAtMatrix(vec3 origin, vec3 target, float roll) {
  vec3 rr = vec3(sin(roll), cos(roll), 0.0);
  vec3 ww = normalize(target - origin);
  vec3 uu = normalize(cross(ww, rr));
  vec3 vv = normalize(cross(uu, ww));

  return mat3(uu, vv, ww);
}

mat4 view_frustum(
    float angle_of_view,
    float aspect_ratio,
    float z_near,
    float z_far
) {
    return mat4(
        vec4(1.0/tan(angle_of_view),           0.0, 0.0, 0.0),
        vec4(0.0, aspect_ratio/tan(angle_of_view),  0.0, 0.0),
        vec4(0.0, 0.0,    (z_far+z_near)/(z_far-z_near), 1.0),
        vec4(0.0, 0.0, -2.0*z_far*z_near/(z_far-z_near), 0.0)
    );
}

void main() {
    mat4 view_matrix = mat4(calcLookAtMatrix(origin, target, 0.0));
    view_matrix[3] = vec4(-origin, 1.0);

    mat4 proj_matrix = view_frustum(radians(45.0), 1.0, 0.01, 2.0);
    gl_Position = proj_matrix * view_matrix * vec4(position, 1.0);
}";

static FS_SRC: &'static str = "
#version 150
out vec4 out_color;

void main() {
    out_color = vec4(0.0, 0.0, 1.0, 1.0);
}";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

pub fn main() {
    // Create an SDL context
    let sdl_context = sdl2::init().unwrap();

    // Get access to the video subsystem
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Create the GL context
    let gl_context = window.gl_create_context().unwrap();

    // Turn off vsync
    video_subsystem.gl_set_swap_interval(SwapInterval::Immediate).unwrap();

    // Load the GL procedure addresses
    let gl = gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    
    // Get the event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
                                 CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program,
                                             CString::new("position")
                                             .unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            std::ptr::null(),
        );
    }

    // Name of the camera uniform
    let origin_name = CString::new("origin").unwrap();
    let target_name = CString::new("target").unwrap();

    let mut origin: [f32; 3] = [0., 0., -1.0];
    let mut target: [f32; 3] = [0., 0., 0.];

    // Start a timer
    let start = Instant::now();
    'running: for frame in 1u64.. {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let origin_loc =
                gl::GetUniformLocation(program, origin_name.as_ptr());
            let target_loc =
                gl::GetUniformLocation(program, target_name.as_ptr());
            gl::Uniform3f(origin_loc, origin[0], origin[1], origin[2]);
            gl::Uniform3f(target_loc, target[0], target[1], target[2]);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Swap the double buffered OpenGL
        window.gl_swap_window();

        if frame & 0xfff == 0 {
            let elapsed = start.elapsed().as_secs_f64();
            print!("FPS {:10.2}\n", frame as f64 / elapsed);
        }

        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    origin[2] += 0.05;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    origin[2] -= 0.05;
                },
                _ => {}
            }
        }

        // Sleep a bit
        //std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}
