use std::io;
use std::fs::File;
use std::io::{Read, BufReader};
use std::ffi::CString;
use std::path::Path;
use std::time::Instant;

use gl::types::*;

use cgmath::{Matrix4, Point3, Vector3, Vector4, Deg, perspective};

use sdl2::event::Event;
use sdl2::video::SwapInterval;
use sdl2::keyboard::Keycode;

// Vertex shader
static VS_SRC: &'static str = "
#version 150
in vec3 position;
uniform mat4 transform_matrix;

out VS_OUT {
    vec3 orig_position;
} vs_out;

void main() {
    vs_out.orig_position = position;
    gl_Position = transform_matrix * vec4(position.x, position.y, position.z, 1.0);
}";

// Fragment shader
static FS_SRC: &'static str = "
#version 150

in vec4 geom_color;
out vec4 out_color;

void main() {
    out_color = geom_color;
}";

// Geometry shader
static GS_SRC: &'static str = "
#version 150

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in VS_OUT {
    vec3 orig_position;
} gs_in[];

out vec4 geom_color;

vec3 GetNormal()
{
   vec3 a = vec3(gs_in[0].orig_position) - vec3(gs_in[1].orig_position);
   vec3 b = vec3(gs_in[2].orig_position) - vec3(gs_in[1].orig_position);
   return abs(normalize(cross(a, b)));
}

void main() {
    vec3 normal = GetNormal();

    gl_Position = gl_in[0].gl_Position;
    geom_color = vec4(0.3 + normal.y / 2);
    EmitVertex();

    gl_Position = gl_in[1].gl_Position;
    geom_color = vec4(0.3 + normal.y / 2);
    EmitVertex();
    
    gl_Position = gl_in[2].gl_Position;
    geom_color = vec4(0.3 + normal.y / 2);
    EmitVertex();

    EndPrimitive();
}
";

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

fn link_program(vs: GLuint, fs: GLuint, gs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::AttachShader(program, gs);
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

/// Load a falkvbo file containing the unique verticies and a list of triangle
/// indicies
pub fn load_falkvbo<P: AsRef<Path>>(path: P)
        -> io::Result<(Vec<(f32, f32, f32)>, Vec<(u32, u32, u32)>)> {
    // Open the file
    let mut fd = BufReader::new(File::open(path)?);

    // Create the vertex and triangle buffers
    let mut verticies = Vec::new();
    let mut triangles = Vec::new();

    // Get the number of verticies
    let mut num_verticies = [0u8; 8];
    fd.read_exact(&mut num_verticies)?;
    let num_verticies = u64::from_le_bytes(num_verticies);

    // Load the vertex data
    for _ in 0..num_verticies {
        let mut x = [0u8; 4];
        fd.read_exact(&mut x)?;
        let x = f32::from_le_bytes(x);
        
        let mut y = [0u8; 4];
        fd.read_exact(&mut y)?;
        let y = f32::from_le_bytes(y);
        
        let mut z = [0u8; 4];
        fd.read_exact(&mut z)?;
        let z = f32::from_le_bytes(z);

        // Save the vertex data
        verticies.push((x, y, z));
    }
    
    // Get the number of triangles
    let mut num_triangles = [0u8; 8];
    fd.read_exact(&mut num_triangles)?;
    let num_triangles = u64::from_le_bytes(num_triangles);

    // Load the triangle data
    for _ in 0..num_triangles {
        let mut a = [0u8; 4];
        fd.read_exact(&mut a)?;
        let a = u32::from_le_bytes(a);
        
        let mut b = [0u8; 4];
        fd.read_exact(&mut b)?;
        let b = u32::from_le_bytes(b);
        
        let mut c = [0u8; 4];
        fd.read_exact(&mut c)?;
        let c = u32::from_le_bytes(c);

        // Save the triangle data
        triangles.push((a, b, c));
    }

    // Return the loaded data!
    Ok((verticies, triangles))
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

    // Get the vertex data and indicies for the data in our object file
    let (vertex_data, triangles) =
        load_falkvbo("mapcombine/kalimdor.falkvbo")
        .expect("Failed to load falkvbo data");

    // Create the GL context
    let _gl = window.gl_create_context().unwrap();

    // Turn off vsync
    video_subsystem.gl_set_swap_interval(SwapInterval::Immediate).unwrap();

    // Set relative mouse mode
    sdl_context.mouse().set_relative_mouse_mode(true);

    // Load the GL procedure addresses
    gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    
    // Get the event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let gs = compile_shader(GS_SRC, gl::GEOMETRY_SHADER);
    let program = link_program(vs, fs, gs);

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
            core::mem::size_of_val(&vertex_data[..]) as isize,
            vertex_data.as_ptr() as *const _,
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

        let mut ele_buffer = 0;
        gl::GenBuffers(1, &mut ele_buffer);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ele_buffer);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
            core::mem::size_of_val(&triangles[..]) as isize,
            triangles.as_ptr() as *const _, gl::STATIC_DRAW);

        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);
    }

    // Name of the camera uniform
    let transform_matrix_name = CString::new("transform_matrix").unwrap();

    let mut head_pos: Point3<f32> = Point3::new(0., 0., 5.);
    let mut head_horiz_angle: f32 = 0.;
    let mut head_vert_angle:  f32 = 0.;
    let mut move_speed:       f32 = 10.;

    // Find the transform matrix location
    let transform_matrix_loc = unsafe {
        gl::GetUniformLocation(program, transform_matrix_name.as_ptr())
    };

    let update_transforms = |origin: &mut Point3<f32>, horiz_angle: f32, vert_angle: f32, movement: f32| {
        let direction = Vector3::new(
            vert_angle.cos() * horiz_angle.sin(),
            vert_angle.sin(),
            vert_angle.cos() * horiz_angle.cos());

        *origin += direction * movement;

        let proj_matrix: Matrix4<f32> =
            perspective(Deg(45.), 1.0, 0.01, 2000000.0);
        let mut view_matrix: Matrix4<f32> =
            Matrix4::look_at(*origin, *origin + direction, Vector3::new(0., 1., 0.));
        let transform_matrix = proj_matrix * view_matrix;

        // Update the uniform
        unsafe {
            gl::UniformMatrix4fv(transform_matrix_loc, 1,
                gl::FALSE as GLboolean,
                AsRef::<[f32; 16]>::as_ref(&transform_matrix).as_ptr());
        }
    };
    
    // Update initial transform state
    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, 0.0);

    // Start a timer
    let start = Instant::now();
    let mut last_status = start;
    'running: for frame in 1u64.. {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, triangles.len() as i32 * 3,
                gl::UNSIGNED_INT, core::ptr::null_mut());
        }

        // Swap the double buffered OpenGL
        window.gl_swap_window();

        if last_status.elapsed().as_secs_f64() >= 1.0 {
            let elapsed = start.elapsed().as_secs_f64();
            print!("FPS {:10.2} | triangles {:10} | verticies {:10}\n",
                   frame as f64 / elapsed,
                   triangles.len(),
                   vertex_data.len());
            last_status = Instant::now();
        }

        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, move_speed);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, -move_speed);
                },
                Event::MouseWheel { x, y, .. } => {
                    if y > 0 {
                        move_speed *= 1.2;
                    } else {
                        move_speed /= 1.2;
                    }
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    let xdel = xrel as f32 / 400.;
                    let ydel = yrel as f32 / 400.;
                    head_horiz_angle = (head_horiz_angle - xdel);
                    head_vert_angle  = (head_vert_angle  - ydel)
                        .min(std::f32::consts::PI / 2. - 0.01)
                        .max(-std::f32::consts::PI / 2. + 0.01);
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, 0.0);
                }
                _ => {}
            }
        }
    }
}
